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

    pub fn add_skill(&self, repo: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(add_skill(repo))
    }

    pub fn install_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.run(install_skill(name))
    }

    pub fn update_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.run(update_skill(name))
    }

    pub fn sync_skills(&self, target: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(sync_skills(target))
    }

    pub fn remove_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run(remove_skill(name))
    }

    pub fn doctor_summary_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.run(doctor_summary_text())
    }
}

pub fn load_skill_summaries() -> Result<Vec<SkillSummary>, Box<dyn std::error::Error>> {
    super::load_skill_summaries()
}

pub async fn add_skill(repo: String) -> Result<(), Box<dyn std::error::Error>> {
    super::add(repo, None, None).await
}

pub async fn install_skill(name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    super::install_selected(name, false, false).await
}

pub async fn update_skill(name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    super::update(name).await
}

pub async fn sync_skills(target: String) -> Result<(), Box<dyn std::error::Error>> {
    super::sync(target, None).await
}

pub async fn remove_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::remove(name).await
}

pub async fn doctor_summary_text() -> Result<String, Box<dyn std::error::Error>> {
    super::commands::doctor_summary().await
}

pub async fn info_skill(name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::commands::info(name).await
}

pub async fn outdated_skills() -> Result<(), Box<dyn std::error::Error>> {
    super::commands::outdated().await
}

pub async fn doctor_skills() -> Result<(), Box<dyn std::error::Error>> {
    super::commands::doctor().await
}

pub async fn clean_generated(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    super::commands::clean(all).await
}
