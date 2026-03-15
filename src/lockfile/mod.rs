use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

pub const LOCKFILE_NAME: &str = "skills.lock.toml";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lockfile {
    pub version: u32,
    pub generated_at: DateTime<Utc>,
    pub config_path: String,
    #[serde(default)]
    pub skills: Vec<LockedSkill>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LockedSkill {
    pub name: String,
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_constraint: Option<String>,
    pub resolved_commit: String,
    pub resolved_tree_hash: String,
    pub resolved_reference: String,
    pub resolved_at: DateTime<Utc>,
}

impl Lockfile {
    pub fn new(config_path: &Path) -> Self {
        Self {
            version: 1,
            generated_at: Utc::now(),
            config_path: config_path.display().to_string(),
            skills: Vec::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn remove_skill(&mut self, name: &str) {
        self.skills.retain(|skill| skill.name != name);
    }

    pub fn get_skill(&self, name: &str) -> Option<&LockedSkill> {
        self.skills.iter().find(|skill| skill.name == name)
    }
}

pub fn lockfile_path_for(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(LOCKFILE_NAME)
}
