use chrono::Utc;
use std::path::{Path, PathBuf};

/// Waterflow Architecture: Plan object for skill creation
/// Transform layer produces this, Effect layer consumes it
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CreatePlan {
    pub target_dir: PathBuf,
    pub skill_name: String,
    pub files: Vec<(PathBuf, String)>, // (relative_path, content)
}

/// Result emitted after successful creation
pub(crate) struct CreatedSkill {
    pub target_dir: PathBuf,
    pub message: String,
}

// ============================================================================
// GUARD: Input validation (pure function, no side effects)
// ============================================================================

fn guard_skill_name(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if name.is_empty() {
        return Err("Skill name cannot be empty".into());
    }

    if name.len() > 50 {
        return Err("Skill name too long (max 50 characters)".into());
    }

    let valid = name
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-');

    if !valid || name.starts_with('-') || name.ends_with('-') || name.contains("--") {
        return Err(
            "Invalid skill name. Use lowercase letters, numbers, and single hyphens only."
                .into(),
        );
    }

    Ok(())
}

fn guard_target_dir(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if target_dir.exists() {
        return Err(format!(
            "Target directory already exists at {}",
            target_dir.display()
        )
        .into());
    }
    Ok(())
}

// ============================================================================
// SHAPE: Path expansion and configuration reading
// ============================================================================

fn shape_target_dir(
    name: &str,
    output_dir: Option<&str>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = match output_dir {
        Some(dir) => PathBuf::from(dir),
        None => workspace_root().unwrap_or(default_create_root()?),
    };

    Ok(root.join(name))
}

pub fn workspace_root() -> Option<PathBuf> {
    let config_path = crate::config::io::find_config().ok()?;
    let config = crate::config::io::load_config(&config_path).ok()?;
    config
        .settings
        .workspace
        .as_deref()
        .map(expand_home_dir)
}

fn default_create_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(dirs::data_dir()
        .ok_or("Could not find data directory")?
        .join("skillmine")
        .join("skills"))
}

fn expand_home_dir(path: &str) -> PathBuf {
    if path == "~" {
        return dirs::home_dir().unwrap_or_else(|| PathBuf::from(path));
    }

    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home_dir) = dirs::home_dir() {
            return home_dir.join(stripped);
        }
    }

    PathBuf::from(path)
}

// ============================================================================
// TRANSFORM: Pure functions that decide WHAT to do (no side effects)
// ============================================================================

fn transform_create_plan(
    name: String,
    output_dir: Option<String>,
) -> Result<CreatePlan, Box<dyn std::error::Error>> {
    // Guard
    guard_skill_name(&name)?;

    // Shape
    let target_dir = shape_target_dir(&name, output_dir.as_deref())?;
    guard_target_dir(&target_dir)?;

    // Transform: Build plan
    let mut files = Vec::new();
    files.push((
        PathBuf::from("SKILL.toml"),
        transform_skill_manifest(&name),
    ));
    files.push((
        PathBuf::from("SKILL.md"),
        transform_skill_markdown(&name),
    ));
    files.push((
        PathBuf::from("README.md"),
        transform_skill_readme(&name, &target_dir),
    ));

    Ok(CreatePlan {
        target_dir,
        skill_name: name,
        files,
    })
}

fn transform_skill_manifest(name: &str) -> String {
    format!(
        r#"manifest_version = "1.0"

[skill]
name = "{name}"
version = "0.1.0"
description = "Use when creating or iterating on the {name} workflow"
type = "technique"
category = "general"
boundary = "Focused on the {name} workflow and its immediate supporting assets"
maturity = "draft"
last_verified = "{last_verified}"
tags = ["{name}"]
topics = ["general-practice"]
non_goals = ["Managing installed skills"]
"#,
        name = name,
        last_verified = Utc::now().date_naive().format("%Y-%m-%d")
    )
}

fn transform_skill_markdown(name: &str) -> String {
    let title = name
        .split('-')
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        r#"---
name: {name}
description: Use when creating or iterating on the {name} workflow
---

# {title}

## Overview

Describe what this skill does and the outcome it helps produce.

## When to Use

- Use when you need the {name} workflow.
- Use when a repeatable process would help.

## Steps

1. Describe the first step.
2. Describe the second step.
3. Describe the verification step.

## Common Mistakes

- Being too broad about the boundary.
- Skipping verification or examples.

## See Also

- Related skills or commands.
"#,
        name = name,
        title = title,
    )
}

fn transform_skill_readme(name: &str, _skill_dir: &Path) -> String {
    format!(
        r#"# {name}

Generated by `skillmine create`.

## Files

- `SKILL.toml` — canonical manifest metadata
- `SKILL.md` — skill instructions and usage guidance
- `README.md` — local authoring notes

## Local lifecycle

Edit files directly in this directory. Changes are immediately reflected after sync.
"#,
        name = name,
    )
}

// ============================================================================
// EFFECT: Side effects only - execute the plan
// ============================================================================

fn effect_execute_create_plan(plan: &CreatePlan) -> Result<(), Box<dyn std::error::Error>> {
    // Create directory
    std::fs::create_dir_all(&plan.target_dir)?;

    // Write all files
    for (relative_path, content) in &plan.files {
        let full_path = plan.target_dir.join(relative_path);
        std::fs::write(&full_path, content)?;
    }

    Ok(())
}

// ============================================================================
// EMIT: Output formatting
// ============================================================================

fn emit_create_success(target_dir: &Path) -> String {
    let path = target_dir.display();
    format!(
        "Created skill package at {}\n\nNext steps:\n  skillmine add {}\n  skillmine install\n  skillmine sync --target=opencode",
        path, path
    )
}

// ============================================================================
// COMMIT: State persistence (if needed)
// ============================================================================

// Note: For create operation, we don't commit to skills.toml yet.
// That's handled separately by `skillmine add` flow.

// ============================================================================
// OBSERVE: Logging hooks (to be added at flow joints)
// ============================================================================

fn observe_plan_created(plan: &CreatePlan) {
    #[cfg(debug_assertions)]
    eprintln!("[OBSERVE] CreatePlan generated: {:?}", plan.target_dir);
}

fn observe_effect_completed(target_dir: &Path) {
    #[cfg(debug_assertions)]
    eprintln!("[OBSERVE] Effect completed: {:?}", target_dir);
}

// ============================================================================
// PUBLIC API: Entry points with complete Waterflow sequence
// ============================================================================

/// Synchronous entry point - returns plan for inspection/testing
#[allow(dead_code)]
pub(crate) fn transform_create_plan_entry(
    name: String,
    output_dir: Option<String>,
) -> Result<CreatePlan, Box<dyn std::error::Error>> {
    transform_create_plan(name, output_dir)
}

/// Full Waterflow: Transform -> Effect -> Emit
pub(crate) fn create_created_skill(
    name: String,
    output_dir: Option<String>,
) -> Result<CreatedSkill, Box<dyn std::error::Error>> {
    // Step 1: Transform - decide WHAT to do
    let plan = transform_create_plan(name, output_dir)?;
    observe_plan_created(&plan);

    // Step 2: Effect - do it
    effect_execute_create_plan(&plan)?;
    observe_effect_completed(&plan.target_dir);

    // Step 3: Emit - report result
    Ok(CreatedSkill {
        message: emit_create_success(&plan.target_dir),
        target_dir: plan.target_dir,
    })
}

/// Async entry point for CLI
pub async fn create(
    name: String,
    output_dir: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(create_created_skill(name, output_dir)?.message)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_skill_name_valid() {
        assert!(guard_skill_name("my-skill").is_ok());
        assert!(guard_skill_name("test123").is_ok());
    }

    #[test]
    fn test_guard_skill_name_invalid() {
        assert!(guard_skill_name("").is_err()); // empty
        assert!(guard_skill_name("MySkill").is_err()); // uppercase
        assert!(guard_skill_name("my_skill").is_err()); // underscore
        assert!(guard_skill_name("-my-skill").is_err()); // leading hyphen
        assert!(guard_skill_name("my-skill-").is_err()); // trailing hyphen
        assert!(guard_skill_name("my--skill").is_err()); // double hyphen
    }

    #[test]
    fn test_transform_skill_manifest() {
        let manifest = transform_skill_manifest("test-skill");
        assert!(manifest.contains("name = \"test-skill\""));
        assert!(manifest.contains("manifest_version"));
    }

    #[test]
    fn test_transform_skill_markdown() {
        let md = transform_skill_markdown("my-test-skill");
        assert!(md.contains("My Test Skill")); // Title case
        assert!(md.contains("name: my-test-skill"));
    }

    #[test]
    fn test_create_plan_structure() {
        let plan = CreatePlan {
            target_dir: PathBuf::from("/tmp/test-skill"),
            skill_name: "test-skill".to_string(),
            files: vec![
                (PathBuf::from("SKILL.toml"), "content".to_string()),
            ],
        };
        assert_eq!(plan.files.len(), 1);
    }
}
