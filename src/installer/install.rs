use crate::error::{Result, SkillmineError};
use crate::config::Config;
use crate::lockfile::Lockfile;
use crate::registry::{version::resolve_version_source, GitClient, ResolvedRef};
use std::fs;
use std::sync::Arc;
use std::path::{Path, PathBuf};

use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use super::effect::{content_path_for_root, copy_dir_all, hard_link_dir, hard_link_or_copy_dir};

#[derive(Debug, Clone)]
pub struct InstallContext {
    pub install_dir: PathBuf,
    pub force: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallOutcomeKind {
    Installed,
    Skipped,
    Error,
}

#[derive(Debug, Clone)]
pub struct InstallOutcome {
    pub name: String,
    pub kind: InstallOutcomeKind,
    pub message: Option<String>,
}

impl InstallOutcome {
    pub fn installed(name: &str, message: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            kind: InstallOutcomeKind::Installed,
            message,
        }
    }

    pub fn skipped(name: &str, message: String) -> Self {
        Self {
            name: name.to_string(),
            kind: InstallOutcomeKind::Skipped,
            message: Some(message),
        }
    }

    pub fn errored(name: &str, message: String) -> Self {
        Self {
            name: name.to_string(),
            kind: InstallOutcomeKind::Error,
            message: Some(message),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InstallSummary {
    pub installed: usize,
    pub skipped: usize,
    pub errors: usize,
}

impl InstallSummary {
    pub fn record(&mut self, outcome: &InstallOutcome) {
        match outcome.kind {
            InstallOutcomeKind::Installed => self.installed += 1,
            InstallOutcomeKind::Skipped => self.skipped += 1,
            InstallOutcomeKind::Error => self.errors += 1,
        }
    }
}

pub fn install_skill_to_store(
    name: &str,
    skill: &crate::config::ConfigSkill,
    config: &Config,
    existing_lockfile: Option<&Lockfile>,
    store: &ContentStore,
    context: &InstallContext,
) -> InstallOutcome {
    let resolved_skill = match resolve_version_source(name, skill, config) {
        Ok(skill) => skill,
        Err(error) => {
            return InstallOutcome::skipped(name, error.to_string());
        }
    };

    if context.verbose {
        println!("Installing '{}'...", name);
    }

    if let Some(broken_repo) =
        crate::cli::state::broken_tmp_repo_path(&resolved_skill, &context.install_dir, name)
    {
        if let Err(error) = std::fs::remove_dir_all(&broken_repo) {
            return InstallOutcome::errored(
                name,
                format!(
                    "Failed to clean broken tmp repo '{}': {}",
                    broken_repo.display(),
                    error
                ),
            );
        }
    }

    match &resolved_skill.source {
        crate::config::SkillSource::GitHub { path, .. } => {
            let resolved = resolve_install_source(name, &resolved_skill, existing_lockfile, context);

            match resolved {
                Ok(resolved_ref) => {
                    let source_path = if let Some(subpath) = path {
                        context.install_dir.join(name).join(subpath)
                    } else {
                        context.install_dir.join(name)
                    };

                    match store.store(&resolved_ref.tree_hash, &source_path) {
                        Ok(_) => {
                            let message = if context.verbose {
                                Some(format!(
                                    "Stored with hash: {}",
                                    &resolved_ref.tree_hash[..8.min(resolved_ref.tree_hash.len())]
                                ))
                            } else {
                                None
                            };
                            InstallOutcome::installed(name, message)
                        }
                        Err(error) => InstallOutcome::errored(
                            name,
                            format!("Failed to store '{}': {}", name, error),
                        ),
                    }
                }
                Err(error) => {
                    InstallOutcome::errored(name, format!("Failed to clone '{}': {}", name, error))
                }
            }
        }
        crate::config::SkillSource::Local { path } => {
            let local_path = PathBuf::from(path);
            if local_path.exists() {
                let hash = format!("local:{}", name);
                match store.store(&hash, &local_path) {
                    Ok(_) => InstallOutcome::installed(name, None),
                    Err(error) => InstallOutcome::errored(
                        name,
                        format!("Failed to store '{}': {}", name, error),
                    ),
                }
            } else {
                InstallOutcome::skipped(name, format!("Local path for '{}' does not exist", name))
            }
        }
        crate::config::SkillSource::Version(_) => InstallOutcome::skipped(
            name,
            format!("Skill '{}' is version-only and not installable yet", name),
        ),
    }
}

fn resolve_install_source(
    name: &str,
    skill: &crate::config::ConfigSkill,
    existing_lockfile: Option<&Lockfile>,
    context: &InstallContext,
) -> std::result::Result<ResolvedRef, Box<dyn std::error::Error>> {
    let resolved =
        if let Some(locked) = existing_lockfile.and_then(|lockfile| lockfile.get_skill(name)) {
            let skill_dir = context.install_dir.join(name);
            if skill_dir.exists() {
                crate::registry::GitClient::resolve_source(&skill.source, &skill_dir)
            } else {
                let mut resolved =
                    GitClient::clone_and_resolve(&skill.source, &skill_dir, !context.force)?;
                resolved.tree_hash = locked.resolved_tree_hash.clone();
                Ok(resolved)
            }
        } else {
            GitClient::clone_and_resolve(
                &skill.source,
                &context.install_dir.join(name),
                !context.force,
            )
        }?;

    Ok(resolved)
}

pub async fn install_many_skills(
    skills: Vec<(String, crate::config::ConfigSkill)>,
    config: Config,
    existing_lockfile: Option<Lockfile>,
    store: ContentStore,
    context: InstallContext,
    concurrency: usize,
) -> Vec<InstallOutcome> {
    let limit = concurrency.max(1);
    let semaphore = Arc::new(Semaphore::new(limit));
    let mut join_set = JoinSet::new();

    for (name, skill) in skills {
        let permit_pool = semaphore.clone();
        let context = context.clone();
        let store = store.clone();
        let config = config.clone();
        let existing_lockfile = existing_lockfile.clone();

        join_set.spawn(async move {
            let _permit = permit_pool.acquire_owned().await.expect("semaphore closed");
            install_skill_to_store(&name, &skill, &config, existing_lockfile.as_ref(), &store, &context)
        });
    }

    let mut outcomes = Vec::new();
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(outcome) => outcomes.push(outcome),
            Err(error) => outcomes.push(InstallOutcome::errored(
                "install-task",
                format!("Install task join failure: {}", error),
            )),
        }
    }

    outcomes
}

#[derive(Clone)]
pub struct ContentStore {
    root: PathBuf,
}

impl ContentStore {
    pub fn default_path() -> Result<PathBuf> {
        dirs::data_dir()
            .map(|dir| dir.join("skillmine").join("store"))
            .ok_or_else(|| SkillmineError::Config("Failed to get data directory".to_string()))
    }

    pub fn new(path: PathBuf) -> Self {
        Self { root: path }
    }

    pub fn default() -> Self {
        Self::new(Self::default_path().unwrap_or_else(|_| PathBuf::from(".skillmine/store")))
    }

    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.root).map_err(SkillmineError::Io)?;
        Ok(())
    }

    fn content_path(&self, tree_hash: &str) -> PathBuf {
        content_path_for_root(&self.root, tree_hash)
    }

    #[cfg(test)]
    pub fn has_content(&self, tree_hash: &str) -> bool {
        self.content_path(tree_hash).exists()
    }

    pub fn store(&self, tree_hash: &str, source_path: &Path) -> Result<PathBuf> {
        let dest_path = self.content_path(tree_hash);

        if dest_path.exists() {
            return Ok(dest_path);
        }

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
        }

        copy_dir_all(source_path, &dest_path)?;

        Ok(dest_path)
    }

    #[allow(dead_code)]
    pub fn store_hard_link(&self, tree_hash: &str, source_path: &Path) -> Result<PathBuf> {
        let dest_path = self.content_path(tree_hash);

        if dest_path.exists() {
            return Ok(dest_path);
        }

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
        }

        hard_link_or_copy_dir(source_path, &dest_path)?;

        Ok(dest_path)
    }

    pub fn get(&self, tree_hash: &str) -> Option<PathBuf> {
        let path = self.content_path(tree_hash);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn link_to(&self, tree_hash: &str, target_path: &Path) -> Result<()> {
        let source_path = self.get(tree_hash).ok_or_else(|| {
            SkillmineError::Installation(format!("Content not found in store: {}", tree_hash))
        })?;

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
        }

        match hard_link_dir(&source_path, target_path) {
            Ok(_) => Ok(()),
            Err(_) => copy_dir_all(&source_path, target_path).map_err(|e| {
                SkillmineError::Installation(format!("Failed to copy to target: {}", e))
            }),
        }
    }

    #[cfg(test)]
    pub fn stats(&self) -> Result<StoreStats> {
        let mut count = 0;
        let mut size = 0u64;

        if self.root.exists() {
            for entry in walkdir::WalkDir::new(&self.root) {
                let entry: walkdir::DirEntry = entry
                    .map_err(|e: walkdir::Error| SkillmineError::Io(std::io::Error::other(e)))?;
                if entry.file_type().is_file() {
                    count += 1;
                    size += entry
                        .metadata()
                        .map_err(|e: walkdir::Error| SkillmineError::Io(std::io::Error::other(e)))?
                        .len();
                }
            }
        }

        Ok(StoreStats { count, size })
    }
}

#[derive(Debug)]
#[cfg(test)]
pub struct StoreStats {
    pub count: u64,
    pub size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_content_path() {
        let store = ContentStore::default();
        let path = store.content_path("abc123def456");
        assert!(path.to_string_lossy().contains("ab/c123def456"));
    }

    #[test]
    fn test_content_path_for_root() {
        let root = PathBuf::from("/tmp/store-root");
        let path = content_path_for_root(&root, "abc123def456");
        assert_eq!(path, root.join("ab").join("c123def456"));
    }

    #[test]
    fn test_store_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let store = ContentStore::new(temp_dir.path().join("store"));
        store.init().unwrap();

        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        let mut file = fs::File::create(source_dir.join("test.txt")).unwrap();
        file.write_all(b"test content").unwrap();

        let hash = "abc123";
        let stored_path = store.store(hash, &source_dir).unwrap();

        assert!(stored_path.exists());
        assert!(stored_path.join("test.txt").exists());

        let retrieved = store.get(hash);
        assert_eq!(retrieved, Some(stored_path));
    }

    #[test]
    fn test_has_content_and_stats() {
        let temp_dir = TempDir::new().unwrap();
        let store = ContentStore::new(temp_dir.path().join("store"));
        store.init().unwrap();

        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        let mut file = fs::File::create(source_dir.join("test.txt")).unwrap();
        file.write_all(b"test content").unwrap();

        let hash = "abc123";
        store.store(hash, &source_dir).unwrap();

        assert!(store.has_content(hash));
        let stats = store.stats().unwrap();
        assert!(stats.count >= 1);
        assert!(stats.size > 0);
    }
}
