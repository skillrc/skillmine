use super::SkillSummary;
use std::future::Future;

pub struct TuiActionExecutor {
    runtime: tokio::runtime::Runtime,
}

impl TuiActionExecutor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            runtime: tokio::runtime::Runtime::new()?,
        })
    }

    fn run<T, F>(&self, future: F) -> Result<T, Box<dyn std::error::Error>>
    where
        F: Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        self.runtime.block_on(future)
    }

    pub fn add_skill(&self, repo: String) -> Result<String, Box<dyn std::error::Error>> {
        self.run(add_skill(repo))
    }

    pub fn install_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.run(install_skill(name))
    }

    pub fn update_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.run(update_skill(name))
    }

    pub fn sync_skills(&self, target: String) -> Result<String, Box<dyn std::error::Error>> {
        self.run(sync_skills(target))
    }

    pub fn remove_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(remove_skill(name))
    }

    pub fn doctor_summary_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.run(doctor_summary_text())
    }

    pub fn enable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(enable_skill(name))
    }

    pub fn disable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(disable_skill(name))
    }

    pub fn unsync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(unsync_skill(name))
    }

    pub fn resync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(resync_skill(name))
    }

    pub fn freeze_skills(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.run(freeze_skills())
    }

    pub fn thaw_skills(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.run(thaw_skills())
    }

    pub fn clean_generated(&self, all: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.run(clean_generated(all))
    }

    pub fn info_skill(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        self.run(info_skill(name))
    }

    pub fn outdated_skills(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.run(outdated_skills())
    }

    pub fn create_skill(
        &self,
        name: String,
        output_dir: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.run(create_skill(name, output_dir))
    }

    pub fn bundle_list_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        bundle_list_text()
    }

    pub fn bundle_current_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        bundle_current_text()
    }

    pub fn model_list_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        model_list_text()
    }

    pub fn model_show_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        model_show_text()
    }
}

pub fn load_skill_summaries() -> Result<Vec<SkillSummary>, Box<dyn std::error::Error>> {
    super::load_skill_summaries()
}

pub async fn add_skill(repo: String) -> Result<String, Box<dyn std::error::Error>> {
    super::add_with_options(repo, false).await
}

pub async fn create_skill(
    name: String,
    output_dir: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    super::create(name, output_dir).await
}

pub async fn install_skill(name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    super::install_selected(name, false, false).await
}

pub async fn update_skill(name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    super::update(name).await
}

pub async fn sync_skills(target: String) -> Result<String, Box<dyn std::error::Error>> {
    let summary_target = target.clone();
    super::sync_with_options(target, None, false)
        .await
        .map(|_| format!("Synced configured skills to the {} runtime target.", summary_target))
}

#[cfg(test)]
#[allow(dead_code)]
pub async fn sync_skills_report_only(target: String) -> Result<String, Box<dyn std::error::Error>> {
    super::sync_with_options(target, None, false).await
}

pub async fn remove_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::remove(name).await
}

pub async fn doctor_summary_text() -> Result<String, Box<dyn std::error::Error>> {
    super::commands::doctor_summary().await
}

pub async fn enable_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::enable(name).await
}

pub async fn disable_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::disable(name).await
}

pub async fn unsync_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::unsync(name).await
}

pub async fn resync_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::resync(name).await
}

pub async fn freeze_skills() -> Result<(), Box<dyn std::error::Error>> {
    super::freeze().await
}

pub async fn thaw_skills() -> Result<(), Box<dyn std::error::Error>> {
    super::thaw().await
}

pub async fn info_skill(name: String) -> Result<String, Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = super::config_and_lockfile()?;
    let tmp_root = super::tmp_root()?;
    let store = crate::installer::ContentStore::default();
    let skill = config
        .skills
        .get(&name)
        .ok_or_else(|| format!("Skill '{}' not found", name))?;
    let summary = super::summary::skill_summary(&name, skill, lockfile.as_ref(), &tmp_root, &store);

    let mut lines = vec![
        format!("Skill: {}", name),
        format!("Source: {}", super::pure::describe_skill_source(skill)),
        format!("Enabled: {}", skill.enabled),
        format!(
            "Outdated: {}",
            super::state::format_outdated_state(super::state::classify_outdated(
                skill,
                lockfile.as_ref().and_then(|lock| lock.get_skill(&name))
            ))
        ),
        format!(
            "Lock: {}",
            super::pure::describe_locked_skill(lockfile.as_ref().and_then(|lock| lock.get_skill(&name)))
        ),
    ];

    if let Some(version) = summary.skill_version {
        lines.push(format!("Skill Version: {}", version));
    }
    if let Some(maturity) = summary.maturity {
        lines.push(format!("Maturity: {}", maturity));
    }
    if let Some(last_verified) = summary.last_verified {
        lines.push(format!("Last Verified: {}", last_verified));
    }
    if let Some(description) = summary.description {
        lines.push(format!("Description: {}", description));
    }

    Ok(lines.join("\n"))
}

pub async fn outdated_skills() -> Result<String, Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = super::config_and_lockfile()?;

    if config.skills.is_empty() {
        return Ok("No skills configured.".to_string());
    }

    let mut lines = Vec::new();
    match lockfile {
        Some(lockfile) => {
            for (name, skill) in &config.skills {
                if !skill.enabled {
                    lines.push(format!("{}: disabled", name));
                    continue;
                }
                let state = super::state::classify_outdated(skill, lockfile.get_skill(name));
                lines.push(format!("{}: {}", name, super::state::format_outdated_state(state)));
            }
        }
        None => {
            for name in config.skills.keys() {
                let skill = config.skills.get(name).unwrap();
                if !skill.enabled {
                    lines.push(format!("{}: disabled", name));
                } else {
                    lines.push(format!("{}: missing-from-lock", name));
                }
            }
        }
    }

    Ok(lines.join("\n"))
}

pub async fn doctor_skills() -> Result<(), Box<dyn std::error::Error>> {
    super::commands::doctor().await
}

pub async fn clean_generated(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    super::commands::clean(all).await
}

pub fn bundle_list_text() -> Result<String, Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;
    Ok(super::bundle::bundle_list(&config.bundles))
}

pub fn bundle_current_text() -> Result<String, Box<dyn std::error::Error>> {
    let opencode_path = default_opencode_config_path();
    super::bundle::bundle_current(&opencode_path)
}

pub fn model_list_text() -> Result<String, Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;
    let opencode_path = default_opencode_config_path();
    super::model::model_list(&config.model_profiles, &opencode_path)
}

pub fn model_show_text() -> Result<String, Box<dyn std::error::Error>> {
    let opencode_path = default_opencode_config_path();
    super::model::model_show(&opencode_path)
}

fn default_opencode_config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("opencode")
        .join("config.json")
}
