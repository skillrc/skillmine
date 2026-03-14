use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SkillmineError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillManifest {
    pub manifest_version: String,
    pub skill: SkillMetadata,
    #[serde(default)]
    pub source: Option<SourceMetadata>,
    #[serde(default)]
    pub compat: Option<CompatMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    #[serde(rename = "type")]
    pub skill_type: String,
    pub category: String,
    pub boundary: String,
    pub maturity: String,
    pub last_verified: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceMetadata {
    pub repository: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompatMetadata {
    pub min_opencode_version: Option<String>,
    pub min_skillmine_version: Option<String>,
}

pub fn manifest_path(root: &Path, subpath: &Option<String>) -> PathBuf {
    match subpath {
        Some(path) => root.join(path).join("SKILL.toml"),
        None => root.join("SKILL.toml"),
    }
}

pub fn load_manifest(root: &Path, subpath: &Option<String>) -> Result<Option<SkillManifest>> {
    let manifest_path = manifest_path(root, subpath);
    if !manifest_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&manifest_path).map_err(SkillmineError::Io)?;
    let manifest: SkillManifest = toml::from_str(&content).map_err(|error| {
        SkillmineError::Config(format!(
            "Invalid manifest at {}: {}",
            manifest_path.display(),
            error
        ))
    })?;

    Ok(Some(manifest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_manifest_from_root() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("SKILL.toml"),
            r#"manifest_version = "1.0"

[skill]
name = "demo"
version = "0.1.0"
description = "Use when testing manifest loading"
type = "technique"
category = "engineering"
boundary = "Focused on manifest parsing"
maturity = "draft"
last_verified = "2026-03-14"
tags = ["testing"]
topics = ["manifest"]
"#,
        )
        .unwrap();

        let manifest = load_manifest(temp_dir.path(), &None).unwrap().unwrap();
        assert_eq!(manifest.skill.name, "demo");
        assert_eq!(manifest.skill.version, "0.1.0");
    }
}
