use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::error::{Result, SkillmineError};
use super::skill::{Skill, SkillId, SkillSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub skills: HashMap<String, SkillConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    #[serde(flatten)]
    pub source: SkillSourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkillSourceConfig {
    GitHub {
        repo: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
    },
    Local {
        path: String,
    },
}

impl Config {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            skills: HashMap::new(),
        }
    }

    pub fn add_skill(&mut self, name: impl Into<String>, config: SkillConfig) {
        self.skills.insert(name.into(), config);
    }

    pub fn get_skill(&self, name: &str) -> Option<&SkillConfig> {
        self.skills.get(name)
    }

    pub fn remove_skill(&mut self, name: &str) -> bool {
        self.skills.remove(name).is_some()
    }

    pub fn parse_github_ref(input: &str) -> Result<(String, Option<String>)> {
        let parts: Vec<&str> = input.split('/').collect();

        if parts.len() < 2 {
            return Err(SkillmineError::validation(format!(
                "Invalid GitHub ref '{}'. Expected format: owner/repo or owner/repo/path",
                input
            )));
        }

        let owner = parts[0];
        let repo = parts[1];

        if owner.is_empty() || repo.is_empty() {
            return Err(SkillmineError::validation("Owner and repo cannot be empty"));
        }

        let path = if parts.len() > 2 {
            Some(parts[2..].join("/"))
        } else {
            None
        };

        Ok((format!("{}/{}", owner, repo), path))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert_eq!(config.version, "1.0");
        assert!(config.skills.is_empty());
    }

    #[test]
    fn test_add_and_get_skill() {
        let mut config = Config::new();

        let skill_config = SkillConfig {
            source: SkillSourceConfig::GitHub {
                repo: "anthropic/skills".to_string(),
                path: Some("git-release".to_string()),
            },
        };

        config.add_skill("git-commit", skill_config);

        assert!(config.get_skill("git-commit").is_some());
        assert!(config.get_skill("nonexistent").is_none());
    }

    #[test]
    fn test_remove_skill() {
        let mut config = Config::new();

        let skill_config = SkillConfig {
            source: SkillSourceConfig::GitHub {
                repo: "test/repo".to_string(),
                path: None,
            },
        };

        config.add_skill("test", skill_config);
        assert!(config.remove_skill("test"));
        assert!(!config.remove_skill("test"));
    }

    #[test]
    fn test_parse_github_ref_simple() {
        let result = Config::parse_github_ref("anthropic/skills").unwrap();
        assert_eq!(result.0, "anthropic/skills");
        assert_eq!(result.1, None);
    }

    #[test]
    fn test_parse_github_ref_with_path() {
        let result = Config::parse_github_ref("anthropic/skills/git-release").unwrap();
        assert_eq!(result.0, "anthropic/skills");
        assert_eq!(result.1, Some("git-release".to_string()));
    }

    #[test]
    fn test_parse_github_ref_nested_path() {
        let result = Config::parse_github_ref("owner/repo/path/to/skill").unwrap();
        assert_eq!(result.0, "owner/repo");
        assert_eq!(result.1, Some("path/to/skill".to_string()));
    }

    #[test]
    fn test_parse_github_ref_invalid() {
        let result = Config::parse_github_ref("invalid");
        assert!(result.is_err());

        let result = Config::parse_github_ref("/repo");
        assert!(result.is_err());
    }

    #[test]
    fn test_toml_serialization() {
        let mut config = Config::new();
        config.add_skill(
            "git-commit",
            SkillConfig {
                source: SkillSourceConfig::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                },
            },
        );

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("git-commit"));
        assert!(toml_str.contains("anthropic/skills"));
    }
}
