use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub concurrency: usize,
    pub timeout: u64,
    pub auto_sync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            concurrency: 5,
            timeout: 300,
            auto_sync: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkillSource {
    GitHub {
        repo: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        commit: Option<String>,
    },
    Local {
        path: String,
    },
    Version(String),
}

impl SkillSource {
    #[allow(dead_code)]
    pub fn repo_name(&self) -> Option<String> {
        match self {
            Self::GitHub { repo, .. } => Some(repo.clone()),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn skill_name(&self, default_name: &str) -> String {
        match self {
            Self::GitHub { repo, path, .. } => {
                if let Some(path) = path {
                    path.split('/')
                        .next_back()
                        .unwrap_or(default_name)
                        .to_string()
                } else {
                    repo.split('/')
                        .next_back()
                        .unwrap_or(default_name)
                        .to_string()
                }
            }
            Self::Local { path } => PathBuf::from(path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(default_name)
                .to_string(),
            Self::Version(_) => default_name.to_string(),
        }
    }
}

impl Default for SkillSource {
    fn default() -> Self {
        Self::Version("*".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ConfigSkill {
    pub source: SkillSource,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum RawConfigSkill {
    Detailed {
        #[serde(flatten)]
        source: SkillSource,
        #[serde(default)]
        name: Option<String>,
    },
    Version(String),
}

impl<'de> Deserialize<'de> for ConfigSkill {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match RawConfigSkill::deserialize(deserializer)? {
            RawConfigSkill::Detailed { source, name } => Ok(Self { source, name }),
            RawConfigSkill::Version(version) => Ok(Self {
                source: SkillSource::Version(version),
                name: None,
            }),
        }
    }
}

impl Serialize for ConfigSkill {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match (&self.source, &self.name) {
            (SkillSource::Version(version), None) => version.serialize(serializer),
            _ => {
                #[derive(Serialize)]
                struct Detailed<'a> {
                    #[serde(flatten)]
                    source: &'a SkillSource,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    name: &'a Option<String>,
                }

                Detailed {
                    source: &self.source,
                    name: &self.name,
                }
                .serialize(serializer)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub skills: BTreeMap<String, ConfigSkill>,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add_skill(&mut self, name: impl Into<String>, skill: ConfigSkill) {
        self.skills.insert(name.into(), skill);
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), String> {
        if self.version != "1.0" && !self.version.starts_with("1.") {
            return Err(format!("unsupported config version: {}", self.version));
        }

        if self.settings.concurrency == 0 {
            return Err("concurrency must be greater than 0".to_string());
        }

        for (name, skill) in &self.skills {
            if name.is_empty() {
                return Err("skill name cannot be empty".to_string());
            }

            if let SkillSource::GitHub {
                repo,
                path,
                branch,
                tag,
                commit,
            } = &skill.source
            {
                if !repo.contains('/') || repo.starts_with('/') || repo.ends_with('/') {
                    return Err(format!(
                        "skill '{}' has invalid repo '{}': expected owner/repo",
                        name, repo
                    ));
                }

                if let Some(path) = path {
                    if path.split('/').any(|segment| segment == "..") {
                        return Err(format!(
                            "skill '{}' has invalid path '{}': parent traversal is not allowed",
                            name, path
                        ));
                    }
                }

                let ref_count = usize::from(branch.is_some())
                    + usize::from(tag.is_some())
                    + usize::from(commit.is_some());

                if ref_count > 1 {
                    return Err(format!(
                        "skill '{}' has conflicting refs: only one of branch, tag, or commit should be specified",
                        name
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn find_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let local_path = PathBuf::from("skills.toml");
        if local_path.exists() {
            return Ok(local_path);
        }

        if let Some(config_dir) = dirs::config_dir() {
            let global_path = config_dir.join("skillmine").join("skills.toml");
            if global_path.exists() {
                return Ok(global_path);
            }
        }

        Err("No configuration file found. Run 'skillmine init' first.".into())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            settings: Settings::default(),
            skills: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_github_skill() {
        let toml_str = r#"
version = "1.0"

[skills]
git-commit = { repo = "anthropic/skills", path = "git-release" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.version, "1.0");
        assert!(config.skills.contains_key("git-commit"));
    }

    #[test]
    fn test_parse_version_only_skill() {
        let toml_str = r#"
version = "1.0"

[skills]
python-testing = "^1.0"
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.skills.contains_key("python-testing"));
        match &config.skills["python-testing"].source {
            SkillSource::Version(version) => assert_eq!(version, "^1.0"),
            _ => panic!("expected version source"),
        }
    }

    #[test]
    fn test_parse_local_skill() {
        let toml_str = r#"
version = "1.0"

[skills]
my-skill = { path = "~/dev/my-skill" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.skills.contains_key("my-skill"));
    }

    #[test]
    fn test_validate_conflicting_refs() {
        let toml_str = r#"
version = "1.0"

[skills]
bad-skill = { repo = "user/repo", branch = "main", tag = "v1.0" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_repo_format() {
        let toml_str = r#"
version = "1.0"

[skills]
bad-skill = { repo = "invalidrepo" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_rejects_parent_traversal() {
        let toml_str = r#"
version = "1.0"

[skills]
bad-skill = { repo = "user/repo", path = "../secret" }
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_version_skill_round_trips_as_string() {
        let mut config = Config::default();
        config.add_skill(
            "python-testing",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );

        let toml = toml::to_string_pretty(&config).unwrap();
        assert!(toml.contains("python-testing = \"^1.0\""));
    }
}
