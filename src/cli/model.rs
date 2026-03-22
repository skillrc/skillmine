use crate::config::ModelProfile;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub type ModelProfileRegistry = BTreeMap<String, ModelProfile>;

// ============================================================================
// GUARD
// ============================================================================

fn guard_profile_exists<'a>(
    name: &str,
    profiles: &'a ModelProfileRegistry,
) -> Result<&'a ModelProfile, Box<dyn std::error::Error>> {
    profiles
        .get(name)
        .ok_or_else(|| format!("Model profile '{}' not found in skills.toml", name).into())
}

// ============================================================================
// EFFECT: Read/write opencode.json
// ============================================================================

fn read_opencode_config(path: &PathBuf) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(serde_json::json!({}))
    }
}

fn write_opencode_config(
    path: &PathBuf,
    config: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Backup before write
    if path.exists() {
        let backup = path.with_extension("json.bak");
        std::fs::copy(path, &backup)?;
    }
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(path, json)?;
    Ok(())
}

// ============================================================================
// EMIT
// ============================================================================

fn emit_model_list(profiles: &ModelProfileRegistry, current: Option<&str>) -> String {
    if profiles.is_empty() {
        return "No model profiles defined. Add [model-profiles.*] to skills.toml.".to_string();
    }

    let mut output = "Model profiles:\n".to_string();
    for (name, profile) in profiles {
        let active = current.map_or(false, |c| {
            profile.model.as_deref().map_or(false, |m| m == c)
        });
        let marker = if active { " (active)" } else { "" };
        output.push_str(&format!("  {}{}\n", name, marker));
        if !profile.description.is_empty() {
            output.push_str(&format!("    {}\n", profile.description));
        }
        if let Some(model) = &profile.model {
            output.push_str(&format!("    model: {}\n", model));
        }
        if let Some(small) = &profile.small_model {
            output.push_str(&format!("    small_model: {}\n", small));
        }
    }
    output
}

fn emit_model_show(opencode_config: &serde_json::Value) -> String {
    let model = opencode_config
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("(not set)");
    let small = opencode_config
        .get("smallModel")
        .and_then(|v| v.as_str());

    let mut output = format!("Current model: {}\n", model);
    if let Some(s) = small {
        output.push_str(&format!("Small model:   {}\n", s));
    }
    output
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Switch to a named model profile
pub fn model_use(
    profile_name: &str,
    profiles: &ModelProfileRegistry,
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    // Guard
    let profile = guard_profile_exists(profile_name, profiles)?;

    // Read existing config
    let mut opencode_config = read_opencode_config(opencode_config_path)?;

    // Apply profile fields
    let mut changed = Vec::new();
    if let Some(model) = &profile.model {
        opencode_config["model"] = serde_json::json!(model);
        changed.push(format!("model = {}", model));
    }
    if let Some(small) = &profile.small_model {
        opencode_config["smallModel"] = serde_json::json!(small);
        changed.push(format!("smallModel = {}", small));
    }

    if changed.is_empty() {
        return Ok(format!(
            "Profile '{}' has no model fields defined.",
            profile_name
        ));
    }

    // Write
    write_opencode_config(opencode_config_path, &opencode_config)?;

    Ok(format!(
        "Switched to profile '{}':\n  {}\n",
        profile_name,
        changed.join("\n  ")
    ))
}

/// List all defined model profiles
pub fn model_list(
    profiles: &ModelProfileRegistry,
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let opencode_config = read_opencode_config(opencode_config_path)?;
    let current = opencode_config
        .get("model")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(emit_model_list(profiles, current.as_deref()))
}

/// Show current model in opencode.json
pub fn model_show(
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let opencode_config = read_opencode_config(opencode_config_path)?;
    Ok(emit_model_show(&opencode_config))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ModelProfile;
    use tempfile::tempdir;

    fn make_profiles() -> ModelProfileRegistry {
        let mut profiles = ModelProfileRegistry::new();
        profiles.insert(
            "focused".to_string(),
            ModelProfile {
                model: Some("anthropic/claude-opus-4-6".to_string()),
                small_model: None,
                description: "Complex tasks".to_string(),
            },
        );
        profiles.insert(
            "fast".to_string(),
            ModelProfile {
                model: Some("anthropic/claude-haiku-4-5".to_string()),
                small_model: None,
                description: "Quick tasks".to_string(),
            },
        );
        profiles
    }

    #[test]
    fn test_model_use_writes_to_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let profiles = make_profiles();
        let result = model_use("focused", &profiles, &config_path).unwrap();

        assert!(result.contains("focused"));
        assert!(result.contains("claude-opus-4-6"));

        let written = std::fs::read_to_string(&config_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&written).unwrap();
        assert_eq!(json["model"], "anthropic/claude-opus-4-6");
    }

    #[test]
    fn test_model_use_creates_backup() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        std::fs::write(&config_path, r#"{"model":"old-model"}"#).unwrap();

        let profiles = make_profiles();
        model_use("fast", &profiles, &config_path).unwrap();

        let backup = config_path.with_extension("json.bak");
        assert!(backup.exists());
        let bak_content = std::fs::read_to_string(&backup).unwrap();
        assert!(bak_content.contains("old-model"));
    }

    #[test]
    fn test_model_use_unknown_profile_errors() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        let profiles = make_profiles();
        let result = model_use("nonexistent", &profiles, &config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_model_list_shows_active() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        std::fs::write(
            &config_path,
            r#"{"model":"anthropic/claude-opus-4-6"}"#,
        )
        .unwrap();

        let profiles = make_profiles();
        let output = model_list(&profiles, &config_path).unwrap();
        assert!(output.contains("focused"));
        assert!(output.contains("(active)"));
    }

    #[test]
    fn test_model_list_empty() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        let profiles = ModelProfileRegistry::new();
        let output = model_list(&profiles, &config_path).unwrap();
        assert!(output.contains("No model profiles defined"));
    }

    #[test]
    fn test_model_show() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        std::fs::write(
            &config_path,
            r#"{"model":"anthropic/claude-sonnet-4-5"}"#,
        )
        .unwrap();

        let output = model_show(&config_path).unwrap();
        assert!(output.contains("claude-sonnet-4-5"));
    }

    #[test]
    fn test_model_show_no_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("nonexistent.json");
        let output = model_show(&config_path).unwrap();
        assert!(output.contains("not set"));
    }
}
