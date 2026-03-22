use std::path::PathBuf;

// ============================================================================
// Waterflow Architecture: instructions management
// Manages the `instructions` array in opencode.json
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
    if path.exists() {
        let backup = path.with_extension("json.bak");
        std::fs::copy(path, &backup)?;
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(config)?)?;
    Ok(())
}

fn get_instructions(config: &serde_json::Value) -> Vec<String> {
    config
        .get("instructions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Add a path to the instructions array
pub fn instructions_add(
    instruction_path: &str,
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = instruction_path.trim();
    if path.is_empty() {
        return Err("Instruction path cannot be empty".into());
    }

    let mut config = read_opencode_config(opencode_config_path)?;
    let mut instructions = get_instructions(&config);

    if instructions.iter().any(|p| p == path) {
        return Ok(format!("'{}' is already in instructions.", path));
    }

    instructions.push(path.to_string());
    config["instructions"] = serde_json::json!(instructions);
    write_opencode_config(opencode_config_path, &config)?;

    Ok(format!("Added '{}' to instructions.", path))
}

/// Remove a path from the instructions array
pub fn instructions_remove(
    instruction_path: &str,
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = instruction_path.trim();

    let mut config = read_opencode_config(opencode_config_path)?;
    let instructions = get_instructions(&config);

    let before = instructions.len();
    let filtered: Vec<String> = instructions
        .into_iter()
        .filter(|p| p != path)
        .collect();

    if filtered.len() == before {
        return Ok(format!("'{}' not found in instructions.", path));
    }

    config["instructions"] = serde_json::json!(filtered);
    write_opencode_config(opencode_config_path, &config)?;

    Ok(format!("Removed '{}' from instructions.", path))
}

/// List all instructions
pub fn instructions_list(
    opencode_config_path: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = read_opencode_config(opencode_config_path)?;
    let instructions = get_instructions(&config);

    if instructions.is_empty() {
        return Ok("No instructions configured.".to_string());
    }

    let mut output = format!("Instructions ({}):\n", instructions.len());
    for (i, path) in instructions.iter().enumerate() {
        output.push_str(&format!("  {}. {}\n", i + 1, path));
    }
    Ok(output)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_add_instruction() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let result = instructions_add("/some/path/SKILL.md", &config_path).unwrap();
        assert!(result.contains("Added"));

        let output = instructions_list(&config_path).unwrap();
        assert!(output.contains("/some/path/SKILL.md"));
    }

    #[test]
    fn test_add_duplicate_is_idempotent() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        instructions_add("/path/SKILL.md", &config_path).unwrap();
        let result = instructions_add("/path/SKILL.md", &config_path).unwrap();
        assert!(result.contains("already in"));

        let config: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&config_path).unwrap(),
        )
        .unwrap();
        assert_eq!(config["instructions"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_remove_instruction() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        instructions_add("/path/a.md", &config_path).unwrap();
        instructions_add("/path/b.md", &config_path).unwrap();

        let result = instructions_remove("/path/a.md", &config_path).unwrap();
        assert!(result.contains("Removed"));

        let output = instructions_list(&config_path).unwrap();
        assert!(!output.contains("/path/a.md"));
        assert!(output.contains("/path/b.md"));
    }

    #[test]
    fn test_remove_nonexistent_is_graceful() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let result = instructions_remove("/not/there.md", &config_path).unwrap();
        assert!(result.contains("not found"));
    }

    #[test]
    fn test_list_empty() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        let output = instructions_list(&config_path).unwrap();
        assert!(output.contains("No instructions"));
    }

    #[test]
    fn test_add_preserves_existing_fields() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        std::fs::write(&config_path, r#"{"model":"existing-model"}"#).unwrap();

        instructions_add("/path/SKILL.md", &config_path).unwrap();

        let config: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&config_path).unwrap(),
        )
        .unwrap();
        assert_eq!(config["model"], "existing-model");
        assert_eq!(config["instructions"].as_array().unwrap().len(), 1);
    }
}
