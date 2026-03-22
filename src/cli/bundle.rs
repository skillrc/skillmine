use crate::config::{BundleSpec, Config};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Waterflow Architecture: Bundle system for workflow activation
/// A Bundle is a named collection of skills, commands, agents, and a model profile
/// that can be activated together.

// ============================================================================
// Domain Types (BundleSpec lives in config::settings for serialization)
// ============================================================================

/// Runtime activation state for a bundle
#[derive(Debug, Clone)]
pub struct BundleActivation {
    pub bundle_name: String,
    pub instructions: Vec<PathBuf>, // Paths to SKILL.md files
    pub model: Option<String>,
    pub default_agent: Option<String>,
}

// ============================================================================
// Bundle Registry (stored in skills.toml)
// ============================================================================

pub type BundleRegistry = BTreeMap<String, BundleSpec>;

// ============================================================================
// TRANSFORM: Build activation plan from bundle spec
// ============================================================================

pub fn transform_bundle_activation(
    bundle_name: &str,
    bundle: &BundleSpec,
    config: &Config,
) -> Result<BundleActivation, Box<dyn std::error::Error>> {
    let mut instructions = Vec::new();

    // Collect skill paths
    for skill_name in &bundle.skills {
        if let Some(skill) = config.skills.get(skill_name) {
            if skill.enabled {
                if let crate::config::SkillSource::Local { path } = &skill.source {
                    let skill_md_path = PathBuf::from(path).join("SKILL.md");
                    if skill_md_path.exists() {
                        instructions.push(skill_md_path);
                    }
                }
            }
        }
    }

    // Determine model from profile
    let model = bundle.model_profile.clone();

    // Determine default agent (first agent in bundle)
    let default_agent = bundle.agents.first().cloned();

    Ok(BundleActivation {
        bundle_name: bundle_name.to_string(),
        instructions,
        model,
        default_agent,
    })
}

// ============================================================================
// EFFECT: Apply bundle to runtime configuration
// ============================================================================

/// Result of applying a bundle
#[derive(Debug, Clone)]
pub struct BundleApplyOutcome {
    pub success: bool,
    pub bundle_name: String,
    pub instructions_count: usize,
    pub model_changed: bool,
    pub message: String,
}

pub fn effect_apply_bundle(
    activation: &BundleActivation,
    opencode_config_path: &PathBuf,
) -> Result<BundleApplyOutcome, Box<dyn std::error::Error>> {
    // Backup existing config
    let backup_path = opencode_config_path.with_extension("json.bak");
    if opencode_config_path.exists() {
        std::fs::copy(opencode_config_path, &backup_path)?;
    }

    // Read existing config or create default
    let mut opencode_config: serde_json::Value = if opencode_config_path.exists() {
        let content = std::fs::read_to_string(opencode_config_path)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::json!({})
    };

    // Update instructions
    let instructions: Vec<String> = activation
        .instructions
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    opencode_config["instructions"] = serde_json::json!(instructions);

    // Update model if specified
    let model_changed = if let Some(model) = &activation.model {
        let old_model = opencode_config.get("model").cloned();
        opencode_config["model"] = serde_json::json!(model);
        old_model != Some(serde_json::json!(model))
    } else {
        false
    };

    // Write updated config
    let config_json = serde_json::to_string_pretty(&opencode_config)?;
    std::fs::write(opencode_config_path, config_json)?;

    Ok(BundleApplyOutcome {
        success: true,
        bundle_name: activation.bundle_name.clone(),
        instructions_count: activation.instructions.len(),
        model_changed,
        message: format!(
            "Activated bundle '{}' with {} instructions",
            activation.bundle_name,
            activation.instructions.len()
        ),
    })
}

// ============================================================================
// EMIT: Format output
// ============================================================================

pub fn emit_bundle_apply_result(outcome: &BundleApplyOutcome) -> String {
    let mut output = format!("✓ {}\n", outcome.message);
    if outcome.model_changed {
        output.push_str("  Model profile updated\n");
    }
    output
}

pub fn emit_bundle_list(bundles: &BundleRegistry) -> String {
    if bundles.is_empty() {
        return "No bundles defined".to_string();
    }

    let mut output = "Available bundles:\n".to_string();
    for (name, spec) in bundles {
        output.push_str(&format!(
            "  {} - {} ({} skills, {} commands, {} agents)\n",
            name,
            spec.description,
            spec.skills.len(),
            spec.commands.len(),
            spec.agents.len()
        ));
    }
    output
}

// ============================================================================
// OBSERVE
// ============================================================================

fn observe_bundle_activation(bundle_name: &str, instructions_count: usize) {
    #[cfg(debug_assertions)]
    eprintln!(
        "[OBSERVE] Bundle '{}' activating with {} instructions",
        bundle_name, instructions_count
    );
}

// ============================================================================
// PUBLIC API
// ============================================================================

pub fn bundle_apply(
    bundle_name: &str,
    bundles: &BundleRegistry,
    config: &Config,
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let bundle = bundles
        .get(bundle_name)
        .ok_or_else(|| format!("Bundle '{}' not found", bundle_name))?;

    // Transform
    let activation = transform_bundle_activation(bundle_name, bundle, config)?;
    observe_bundle_activation(bundle_name, activation.instructions.len());

    // Effect
    let outcome = effect_apply_bundle(&activation, opencode_config_path)?;

    // Emit
    Ok(emit_bundle_apply_result(&outcome))
}

pub fn bundle_list(bundles: &BundleRegistry) -> String {
    emit_bundle_list(bundles)
}

pub fn bundle_clear(opencode_config_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    // Backup
    let backup_path = opencode_config_path.with_extension("json.bak");
    if opencode_config_path.exists() {
        std::fs::copy(opencode_config_path, &backup_path)?;
    }

    // Read existing
    let mut opencode_config: serde_json::Value = if opencode_config_path.exists() {
        let content = std::fs::read_to_string(opencode_config_path)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::json!({})
    };

    // Clear instructions
    let empty: Vec<String> = Vec::new();
    opencode_config["instructions"] = serde_json::json!(empty);

    // Write
    let config_json = serde_json::to_string_pretty(&opencode_config)?;
    std::fs::write(opencode_config_path, config_json)?;

    Ok("✓ Bundle deactivated (instructions cleared)".to_string())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_spec_default() {
        let spec = BundleSpec {
            description: "Test bundle".to_string(),
            skills: vec!["skill1".to_string(), "skill2".to_string()],
            commands: vec!["cmd1".to_string()],
            agents: vec![],
            model_profile: Some("focused".to_string()),
        };
        assert_eq!(spec.skills.len(), 2);
    }

    #[test]
    fn test_emit_bundle_list_empty() {
        let bundles = BundleRegistry::new();
        assert_eq!(emit_bundle_list(&bundles), "No bundles defined");
    }
}
