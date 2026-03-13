use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const LOCKFILE_NAME: &str = "skills.lock.toml";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Lockfile {
    version: u32,
    generated_at: DateTime<Utc>,
    config_path: String,
    #[serde(default)]
    skills: Vec<LockedSkill>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct LockedSkill {
    name: String,
    source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_constraint: Option<String>,
    resolved_commit: String,
    resolved_tree_hash: String,
    resolved_reference: String,
    resolved_at: DateTime<Utc>,
}

type ConfigBundle = (
    PathBuf,
    crate::config::Config,
    PathBuf,
    Option<Lockfile>,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SkillStatus {
    Configured,
    Installed,
    Cached,
    Locked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutdatedState {
    UpToDate,
    LockDrift,
    MissingFromLock,
    Local,
    CacheMissing,
    TmpMissing,
    Pinned,
    SourceMismatch,
}

impl Lockfile {
    fn new(config_path: &Path) -> Self {
        Self {
            version: 1,
            generated_at: Utc::now(),
            config_path: config_path.display().to_string(),
            skills: Vec::new(),
        }
    }

    fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    fn remove_skill(&mut self, name: &str) {
        self.skills.retain(|skill| skill.name != name);
    }

    fn get_skill(&self, name: &str) -> Option<&LockedSkill> {
        self.skills.iter().find(|skill| skill.name == name)
    }
}

fn lockfile_path_for(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(LOCKFILE_NAME)
}

fn tmp_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(dirs::data_dir()
        .ok_or("Could not find data directory")?
        .join("skillmine")
        .join("tmp"))
}

fn data_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(dirs::data_dir()
        .ok_or("Could not find data directory")?
        .join("skillmine"))
}

fn config_and_lockfile() -> Result<ConfigBundle, Box<dyn std::error::Error>> {
    use crate::config::Config;

    let config_path = Config::find_config()?;
    let config = Config::load(&config_path)?;
    let lockfile_path = lockfile_path_for(&config_path);
    let lockfile = if lockfile_path.exists() {
        Some(Lockfile::load(&lockfile_path)?)
    } else {
        None
    };

    Ok((config_path, config, lockfile_path, lockfile))
}

fn build_locked_skill(
    name: &str,
    skill: &crate::config::ConfigSkill,
    tmp_root: &Path,
) -> Result<LockedSkill, Box<dyn std::error::Error>> {
    match &skill.source {
        crate::config::SkillSource::GitHub {
            repo,
            path,
            branch,
            tag,
            commit,
        } => {
            let requested_commit = commit.clone();
            let repo_dir = tmp_root.join(name);

            let (resolved_commit, resolved_tree_hash, resolved_reference) = if let Some(commit) = commit {
                if repo_dir.exists() {
                    let tree_hash = crate::registry::GitClient::get_path_tree_hash(&repo_dir, path)?;
                    (commit.clone(), tree_hash, commit.clone())
                } else {
                    return Err(format!("Skill '{}' is pinned but not installed locally; run install first", name).into());
                }
            } else if repo_dir.exists() {
                let resolved = crate::registry::GitClient::resolve_source(&skill.source, &repo_dir)?;
                (resolved.commit, resolved.tree_hash, resolved.reference)
            } else {
                return Err(format!("Skill '{}' is unresolved; run install first before freezing", name).into());
            };

            Ok(LockedSkill {
                name: name.to_string(),
                source_type: "github".to_string(),
                repo: Some(repo.clone()),
                path: path.clone(),
                requested_branch: branch.clone(),
                requested_tag: tag.clone(),
                requested_commit,
                local_path: None,
                version_constraint: None,
                resolved_commit,
                resolved_tree_hash,
                resolved_reference,
                resolved_at: Utc::now(),
            })
        }
        crate::config::SkillSource::Local { path } => {
            let local_path = PathBuf::from(path);
            if !local_path.exists() {
                return Err(format!("Local skill '{}' does not exist at {}", name, local_path.display()).into());
            }

            let (resolved_commit, resolved_tree_hash, resolved_reference) =
                if local_path.join(".git").exists() {
                    let tree_hash = crate::registry::GitClient::get_path_tree_hash(&local_path, &None)?;
                    let resolved = crate::registry::GitClient::resolve_local_head(&local_path)?;
                    (resolved.commit, tree_hash, "local-git".to_string())
                } else {
                    (
                        "local".to_string(),
                        format!("path:{}", local_path.display()),
                        "local".to_string(),
                    )
                };

            Ok(LockedSkill {
                name: name.to_string(),
                source_type: "local".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: Some(path.clone()),
                version_constraint: None,
                resolved_commit,
                resolved_tree_hash,
                resolved_reference,
                resolved_at: Utc::now(),
            })
        }
        crate::config::SkillSource::Version(version) => Ok(LockedSkill {
            name: name.to_string(),
            source_type: "version".to_string(),
            repo: None,
            path: None,
            requested_branch: None,
            requested_tag: None,
            requested_commit: None,
            local_path: None,
            version_constraint: Some(version.clone()),
            resolved_commit: version.clone(),
            resolved_tree_hash: version.clone(),
            resolved_reference: "version".to_string(),
            resolved_at: Utc::now(),
        }),
    }
}

fn refresh_lockfile_from_current_state(
    config_path: &Path,
    config: &crate::config::Config,
    previous_lockfile: Option<&Lockfile>,
) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_root = tmp_root()?;
    let lockfile_path = lockfile_path_for(config_path);
    let mut lockfile = Lockfile::new(config_path);

    for (name, skill) in &config.skills {
        if let Some(existing) = previous_lockfile.and_then(|lock| lock.get_skill(name)) {
            if matches!(skill.source, crate::config::SkillSource::GitHub { .. }) {
                lockfile.skills.push(existing.clone());
                continue;
            }
        }

        if let Ok(locked_skill) = build_locked_skill(name, skill, &tmp_root) {
            lockfile.skills.push(locked_skill);
        }
    }

    lockfile.save(&lockfile_path)?;
    Ok(())
}

fn cached_or_tmp_source_for_skill(
    name: &str,
    skill: &crate::config::ConfigSkill,
    lockfile: Option<&Lockfile>,
    store: &crate::installer::ContentStore,
    tmp_root: &Path,
) -> Option<PathBuf> {
    if let Some(locked) = lockfile.and_then(|lock| lock.get_skill(name)) {
        if let Some(cached) = store.get(&locked.resolved_tree_hash) {
            return Some(cached);
        }
    }

    match &skill.source {
        crate::config::SkillSource::GitHub {
            path: Some(subpath), ..
        } => Some(tmp_root.join(name).join(subpath)),
        crate::config::SkillSource::GitHub { .. } => Some(tmp_root.join(name)),
        crate::config::SkillSource::Local { path } => Some(PathBuf::from(path)),
        crate::config::SkillSource::Version(_) => None,
    }
}

fn broken_tmp_repo_path(skill: &crate::config::ConfigSkill, tmp_root: &Path, name: &str) -> Option<PathBuf> {
    match &skill.source {
        crate::config::SkillSource::GitHub { .. } => {
            let repo_dir = tmp_root.join(name);
            if repo_dir.exists() && !crate::registry::GitClient::has_resolvable_head(&repo_dir) {
                Some(repo_dir)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn skill_statuses(
    name: &str,
    skill: &crate::config::ConfigSkill,
    lockfile: Option<&Lockfile>,
    tmp_root: &Path,
    store: &crate::installer::ContentStore,
) -> Vec<SkillStatus> {
    let mut statuses = vec![SkillStatus::Configured];
    let locked_skill = lockfile.and_then(|lock| lock.get_skill(name));

    let installed_path = match &skill.source {
        crate::config::SkillSource::GitHub {
            path: Some(subpath), ..
        } => tmp_root.join(name).join(subpath),
        crate::config::SkillSource::GitHub { .. } => tmp_root.join(name),
        crate::config::SkillSource::Local { path } => PathBuf::from(path),
        crate::config::SkillSource::Version(_) => PathBuf::new(),
    };

    match &skill.source {
        crate::config::SkillSource::GitHub { .. } => {
            if locked_skill.is_some() && !installed_path.as_os_str().is_empty() && installed_path.exists() {
                statuses.push(SkillStatus::Installed);
            }
        }
        crate::config::SkillSource::Local { .. } => {
            if !installed_path.as_os_str().is_empty() && installed_path.exists() {
                statuses.push(SkillStatus::Installed);
            }
        }
        crate::config::SkillSource::Version(_) => {}
    }

    if let Some(lockfile) = locked_skill {
        statuses.push(SkillStatus::Locked);
        if store.get(&lockfile.resolved_tree_hash).is_some() {
            statuses.push(SkillStatus::Cached);
        }
    }

    statuses
}

fn format_statuses(statuses: &[SkillStatus]) -> String {
    statuses
        .iter()
        .map(|status| match status {
            SkillStatus::Configured => "configured",
            SkillStatus::Installed => "installed",
            SkillStatus::Cached => "cached",
            SkillStatus::Locked => "locked",
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn classify_outdated(
    skill: &crate::config::ConfigSkill,
    locked: Option<&LockedSkill>,
) -> OutdatedState {
    let Some(locked) = locked else {
        return OutdatedState::MissingFromLock;
    };

    match (&skill.source, locked.source_type.as_str()) {
        (crate::config::SkillSource::GitHub { commit, .. }, "github") => {
            if commit.is_some() {
                OutdatedState::Pinned
            } else {
                let tmp_repo = match dirs::data_dir() {
                    Some(dir) => dir.join("skillmine").join("tmp").join(&locked.name),
                    None => return OutdatedState::TmpMissing,
                };

                if !tmp_repo.exists() || !crate::registry::GitClient::has_resolvable_head(&tmp_repo) {
                    OutdatedState::TmpMissing
                } else {
                    match crate::registry::GitClient::resolve_local_head(&tmp_repo) {
                        Ok(resolved)
                            if resolved.commit == locked.resolved_commit
                                && resolved.tree_hash == locked.resolved_tree_hash =>
                        {
                            OutdatedState::UpToDate
                        }
                        Ok(_) => OutdatedState::LockDrift,
                        Err(_) => OutdatedState::TmpMissing,
                    }
                }
            }
        }
        (crate::config::SkillSource::Version(version), "version") => {
            if locked.resolved_tree_hash == "missing-cache" {
                OutdatedState::CacheMissing
            } else if version == &locked.resolved_commit {
                OutdatedState::UpToDate
            } else {
                OutdatedState::LockDrift
            }
        }
        (crate::config::SkillSource::Local { path }, "local") => {
            let local_path = Path::new(path);
            if local_path.join(".git").exists() {
                match crate::registry::GitClient::resolve_local_head(local_path) {
                    Ok(resolved) => {
                        if resolved.commit == locked.resolved_commit
                            && resolved.tree_hash == locked.resolved_tree_hash
                        {
                            OutdatedState::UpToDate
                        } else {
                            OutdatedState::LockDrift
                        }
                    }
                    Err(_) => OutdatedState::Local,
                }
            } else {
                OutdatedState::Local
            }
        }
        _ => OutdatedState::SourceMismatch,
    }
}

fn format_outdated_state(state: OutdatedState) -> &'static str {
    match state {
        OutdatedState::UpToDate => "up-to-date",
        OutdatedState::LockDrift => "lock-drift",
        OutdatedState::MissingFromLock => "missing-from-lock",
        OutdatedState::Local => "local",
        OutdatedState::CacheMissing => "cache-missing",
        OutdatedState::TmpMissing => "tmp-missing",
        OutdatedState::Pinned => "pinned",
        OutdatedState::SourceMismatch => "source-mismatch",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagnosticLevel {
    Pass,
    Warn,
    Fail,
}

impl DiagnosticLevel {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warn => "WARN",
            Self::Fail => "FAIL",
        }
    }
}

fn print_diagnostic(level: DiagnosticLevel, message: impl AsRef<str>) {
    println!("{}: {}", level.as_str(), message.as_ref());
}

fn skill_health_lines(
    name: &str,
    skill: &crate::config::ConfigSkill,
    lockfile: Option<&Lockfile>,
    tmp_path: &Path,
    store: &crate::installer::ContentStore,
) -> Vec<(DiagnosticLevel, String)> {
    let mut lines = Vec::new();

    let state = classify_outdated(skill, lockfile.and_then(|lock| lock.get_skill(name)));
    lines.push((
        match state {
            OutdatedState::LockDrift | OutdatedState::SourceMismatch => DiagnosticLevel::Warn,
            OutdatedState::CacheMissing | OutdatedState::TmpMissing => DiagnosticLevel::Warn,
            OutdatedState::UpToDate | OutdatedState::Pinned | OutdatedState::Local | OutdatedState::MissingFromLock => DiagnosticLevel::Pass,
        },
        format!("skill '{}' state: {}", name, format_outdated_state(state)),
    ));

    if let Some(locked) = lockfile.and_then(|lock| lock.get_skill(name)) {
        if store.get(&locked.resolved_tree_hash).is_some() {
            lines.push((
                DiagnosticLevel::Pass,
                format!("skill '{}' cache present", name),
            ));
        }
    }

    if matches!(skill.source, crate::config::SkillSource::GitHub { .. }) {
        let repo_dir = tmp_path.join(name);
        if repo_dir.exists() && crate::registry::GitClient::has_resolvable_head(&repo_dir) {
            lines.push((
                DiagnosticLevel::Pass,
                format!("skill '{}' tmp clone healthy", name),
            ));
        }
    }

    if let crate::config::SkillSource::Local { path } = &skill.source {
        let exists = Path::new(path).exists();
        lines.push((
            if exists {
                DiagnosticLevel::Pass
            } else {
                DiagnosticLevel::Warn
            },
            format!("skill '{}' local path exists: {}", name, exists),
        ));
    }

    lines
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct DiagnosticSummary {
    pass: usize,
    warn: usize,
    fail: usize,
}

impl DiagnosticSummary {
    #[allow(dead_code)]
    fn record(&mut self, level: DiagnosticLevel) {
        match level {
            DiagnosticLevel::Pass => self.pass += 1,
            DiagnosticLevel::Warn => self.warn += 1,
            DiagnosticLevel::Fail => self.fail += 1,
        }
    }
}

fn short_hash(value: &str) -> String {
    value.chars().take(8).collect()
}

fn describe_skill_source(skill: &crate::config::ConfigSkill) -> String {
    match &skill.source {
        crate::config::SkillSource::GitHub {
            repo,
            path,
            branch,
            tag,
            commit,
        } => {
            let mut parts = vec![format!("github:{}", repo)];
            if let Some(path) = path {
                parts.push(format!("path={}", path));
            }
            if let Some(branch) = branch {
                parts.push(format!("branch={}", branch));
            }
            if let Some(tag) = tag {
                parts.push(format!("tag={}", tag));
            }
            if let Some(commit) = commit {
                parts.push(format!("commit={}", short_hash(commit)));
            }
            parts.join(" ")
        }
        crate::config::SkillSource::Local { path } => format!("local:{}", path),
        crate::config::SkillSource::Version(version) => format!("version:{}", version),
    }
}

fn describe_locked_skill(locked: Option<&LockedSkill>) -> String {
    match locked {
        Some(locked) => format!(
            "locked={} tree={} ref={}",
            short_hash(&locked.resolved_commit),
            short_hash(&locked.resolved_tree_hash),
            locked.resolved_reference
        ),
        None => "locked=none".to_string(),
    }
}

pub async fn init(local: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = if local {
        PathBuf::from("skills.toml")
    } else {
        dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("skillmine")
            .join("skills.toml")
    };

    if config_path.exists() {
        return Err(format!("Configuration already exists at {:?}", config_path).into());
    }

    let config_dir = config_path.parent().ok_or("Invalid config path")?;
    std::fs::create_dir_all(config_dir)?;

    let default_config = r#"version = "1.0"

[settings]
concurrency = 5
timeout = 300
auto_sync = false

[skills]
# Add your skills here
# Example:
# git-commit = { repo = "anthropic/skills", path = "git-release" }
"#;

    std::fs::write(&config_path, default_config)?;

    println!("✓ Created configuration at {:?}", config_path);
    println!("\nNext steps:");
    println!("  1. Edit the file to add your skills");
    println!("  2. Run 'skillmine install' to install them");

    Ok(())
}

pub async fn add(
    repo: String,
    branch: Option<String>,
    tag: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::{Config, ConfigSkill, SkillSource};

    let config_path = Config::find_config()?;
    let mut config = Config::load(&config_path)?;

    let (repo_name, path) = crate::registry::GitClient::parse_github_ref(&repo)
        .ok_or("Invalid repository format. Use 'owner/repo' or 'owner/repo/path'")?;

    let skill_name = path
        .clone()
        .unwrap_or_else(|| repo_name.split('/').next_back().unwrap_or(&repo).to_string());

    let skill_source = SkillSource::GitHub {
        repo: repo_name,
        path,
        branch: branch.clone(),
        tag: tag.clone(),
        commit: None,
    };

    let skill = ConfigSkill {
        source: skill_source,
        name: Some(skill_name.clone()),
    };

    config.add_skill(&skill_name, skill);
    config.save(&config_path)?;

    println!("✓ Added '{}' to skills.toml", skill_name);
    println!("  Run 'skillmine install' to install it");

    Ok(())
}

pub async fn install(force: bool, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::Config;
    use crate::installer::ContentStore;
    use crate::registry::GitClient;

    let config_path = Config::find_config()?;
    let config = Config::load(&config_path)?;
    let lockfile_path = lockfile_path_for(&config_path);
    let existing_lockfile = if lockfile_path.exists() {
        Some(Lockfile::load(&lockfile_path)?)
    } else {
        None
    };

    if config.skills.is_empty() {
        println!("No skills configured. Run 'skillmine add <repo>' to add skills.");
        return Ok(());
    }

    let store = ContentStore::default();
    store.init()?;

    let install_dir = tmp_root()?;
    std::fs::create_dir_all(&install_dir)?;

    let mut installed = 0;
    let mut skipped = 0;
    let mut errors = 0;

    for (name, skill) in &config.skills {
        if verbose {
            println!("Installing '{}'...", name);
        }

        if let Some(broken_repo) = broken_tmp_repo_path(skill, &install_dir, name) {
            std::fs::remove_dir_all(&broken_repo)?;
        }

        match &skill.source {
            crate::config::SkillSource::GitHub { path, .. } => {
                let resolved = if let Some(locked) = existing_lockfile
                    .as_ref()
                    .and_then(|lockfile| lockfile.get_skill(name))
                {
                    let skill_dir = install_dir.join(name);
                    if skill_dir.exists() {
                        crate::registry::GitClient::resolve_source(&skill.source, &skill_dir)
                    } else {
                        let mut resolved = GitClient::clone_and_resolve(&skill.source, &skill_dir, !force)?;
                        resolved.tree_hash = locked.resolved_tree_hash.clone();
                        Ok(resolved)
                    }
                } else {
                    GitClient::clone_and_resolve(&skill.source, &install_dir.join(name), !force)
                };

                match resolved {
                    Ok(resolved_ref) => {
                        let source_path = if let Some(subpath) = path {
                            install_dir.join(name).join(subpath)
                        } else {
                            install_dir.join(name)
                        };

                        if let Err(error) = store.store(&resolved_ref.tree_hash, &source_path) {
                            eprintln!("Failed to store '{}': {}", name, error);
                            errors += 1;
                        } else {
                            if verbose {
                                println!(
                                    "  ✓ Stored with hash: {}",
                                    &resolved_ref.tree_hash[..8.min(resolved_ref.tree_hash.len())]
                                );
                            }
                            installed += 1;
                        }
                    }
                    Err(error) => {
                        eprintln!("Failed to clone '{}': {}", name, error);
                        errors += 1;
                    }
                }
            }
            crate::config::SkillSource::Local { path } => {
                let local_path = PathBuf::from(path);
                if local_path.exists() {
                    let hash = format!("local:{}", name);
                    store.store(&hash, &local_path)?;
                    installed += 1;
                } else {
                    eprintln!("Skipping '{}': local path does not exist", name);
                    skipped += 1;
                }
            }
            crate::config::SkillSource::Version(_) => {
                eprintln!("Skipping '{}': version-only sources are not installable yet", name);
                skipped += 1;
            }
        }
    }

    println!("\nInstallation complete:");
    println!("  {} installed", installed);
    println!("  {} skipped", skipped);
    if errors > 0 {
        println!("  {} errors", errors);
    }

    refresh_lockfile_from_current_state(&config_path, &config, existing_lockfile.as_ref())?;

    Ok(())
}

pub async fn sync(target: String, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::Config;
    use crate::installer::ContentStore;
    use crate::registry::GitClient;
    use std::os::unix::fs::symlink;

    let config_path = Config::find_config()?;
    let config = Config::load(&config_path)?;
    let lockfile_path = lockfile_path_for(&config_path);
    let lockfile = if lockfile_path.exists() {
        Some(Lockfile::load(&lockfile_path)?)
    } else {
        None
    };

    if config.skills.is_empty() {
        println!("No skills configured.");
        return Ok(());
    }

    let target_dir = if let Some(custom_path) = path {
        std::path::PathBuf::from(custom_path)
    } else {
        match target.as_str() {
            "claude" => dirs::home_dir()
                .ok_or("Could not find home directory")?
                .join(".claude")
                .join("skills"),
            "opencode" => dirs::config_dir()
                .ok_or("Could not find config directory")?
                .join("opencode")
                .join("skills"),
            _ => return Err(format!("Unknown target: {}. Use 'claude' or 'opencode'", target).into()),
        }
    };

    let store = ContentStore::default();
    let tmp_dir = tmp_root()?;

    std::fs::create_dir_all(&target_dir)?;

    let mut synced = 0;
    let mut errors = 0;

    for (name, skill) in &config.skills {
        let skill_target = target_dir.join(name);

        if skill_target.exists() {
            if skill_target.is_symlink() {
                std::fs::remove_file(&skill_target)?;
            } else if skill_target.is_dir() {
                std::fs::remove_dir_all(&skill_target)?;
            }
        }

        match &skill.source {
            crate::config::SkillSource::GitHub { .. } => {
                if let Some(source) = cached_or_tmp_source_for_skill(
                    name,
                    skill,
                    lockfile.as_ref(),
                    &store,
                    &tmp_dir,
                ) {
                    if let Err(error) = symlink(&source, &skill_target) {
                        eprintln!("Failed to symlink '{}': {}", name, error);
                        errors += 1;
                    } else {
                        println!("✓ Synced '{}' -> {}", name, skill_target.display());
                        synced += 1;
                    }
                } else {
                    let resolved = GitClient::clone_and_resolve(&skill.source, &tmp_dir.join(name), true);

                    match resolved {
                        Ok(resolved_ref) => {
                            let source_path = store.get(&resolved_ref.tree_hash).or_else(|| {
                                let source = match &skill.source {
                                    crate::config::SkillSource::GitHub {
                                        path: Some(subpath), ..
                                    } => tmp_dir.join(name).join(subpath),
                                    _ => tmp_dir.join(name),
                                };

                                store.store(&resolved_ref.tree_hash, &source).ok()
                            });

                            if let Some(source) = source_path {
                                if let Err(error) = symlink(&source, &skill_target) {
                                    eprintln!("Failed to symlink '{}': {}", name, error);
                                    errors += 1;
                                } else {
                                    println!("✓ Synced '{}' -> {}", name, skill_target.display());
                                    synced += 1;
                                }
                            } else {
                                eprintln!("Failed to get source for '{}'", name);
                                errors += 1;
                            }
                        }
                        Err(error) => {
                            eprintln!("Failed to resolve '{}': {}", name, error);
                            errors += 1;
                        }
                    }
                }
            }
            crate::config::SkillSource::Local { path: local_path } => {
                let source = std::path::PathBuf::from(local_path);
                if source.exists() {
                    if let Err(error) = symlink(&source, &skill_target) {
                        eprintln!("Failed to symlink '{}': {}", name, error);
                        errors += 1;
                    } else {
                        println!("✓ Synced '{}' -> {}", name, skill_target.display());
                        synced += 1;
                    }
                } else {
                    eprintln!("Local path for '{}' does not exist: {}", name, source.display());
                    errors += 1;
                }
            }
            _ => {
                eprintln!("Skipping '{}': unsupported source type", name);
            }
        }
    }

    println!("\nSync complete:");
    println!("  {} synced to {}", synced, target_dir.display());
    if errors > 0 {
        println!("  {} errors", errors);
    }

    Ok(())
}

pub async fn freeze() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config, lockfile_path, _) = config_and_lockfile()?;
    let tmp_root = tmp_root()?;
    let mut lockfile = Lockfile::new(&config_path);

    for (name, skill) in &config.skills {
        lockfile.skills.push(build_locked_skill(name, skill, &tmp_root)?);
    }

    lockfile.save(&lockfile_path)?;
    println!("✓ Wrote lockfile to {}", lockfile_path.display());
    Ok(())
}

pub async fn thaw() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, mut config, lockfile_path, lockfile) = config_and_lockfile()?;
    let lockfile = lockfile.ok_or("No lockfile found. Run 'skillmine freeze' first.")?;

    for locked in &lockfile.skills {
        let Some(skill) = config.skills.get_mut(&locked.name) else {
            return Err(format!("Locked skill '{}' not found in config", locked.name).into());
        };

        match (&mut skill.source, locked.source_type.as_str()) {
            (
                crate::config::SkillSource::GitHub {
                    branch,
                    tag,
                    commit,
                    ..
                },
                "github",
            ) => {
                *branch = None;
                *tag = None;
                *commit = Some(locked.resolved_commit.clone());
            }
            (crate::config::SkillSource::Local { .. }, "local") => {}
            (crate::config::SkillSource::Version(_), "version") => {}
            _ => {
                return Err(format!("Source mismatch for locked skill '{}'", locked.name).into());
            }
        }
    }

    config.save(&config_path)?;
    println!("✓ Applied lockfile from {}", lockfile_path.display());
    Ok(())
}

pub async fn list(detailed: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = config_and_lockfile()?;
    let tmp_root = tmp_root()?;
    let store = crate::installer::ContentStore::default();

    if config.skills.is_empty() {
        println!("No skills configured.");
        return Ok(());
    }

    for (name, skill) in &config.skills {
        let statuses = skill_statuses(name, skill, lockfile.as_ref(), &tmp_root, &store);
        let outdated_state = format_outdated_state(classify_outdated(skill, lockfile.as_ref().and_then(|lock| lock.get_skill(name))));
        let locked_summary = describe_locked_skill(lockfile.as_ref().and_then(|lock| lock.get_skill(name)));
        if detailed {
            println!("- {} [{}]", name, format_statuses(&statuses));
            println!("  source: {}", describe_skill_source(skill));
            println!("  outdated: {}", outdated_state);
            println!("  {}", locked_summary);
            println!("  tmp: {}", tmp_root.join(name).exists());
        } else {
            println!(
                "{} [{}] [{}]",
                name,
                format_statuses(&statuses),
                outdated_state
            );
        }
    }

    Ok(())
}

pub async fn update(_skill: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, mut config, lockfile_path, lockfile) = config_and_lockfile()?;
    let mut lockfile = lockfile.ok_or("No lockfile found. Run 'skillmine freeze' first.")?;

    let selected_skill = _skill.as_deref();

    for locked in &mut lockfile.skills {
        if let Some(selected) = selected_skill {
            if locked.name != selected {
                continue;
            }
        }

        let Some(skill) = config.skills.get_mut(&locked.name) else {
            continue;
        };

        match (&mut skill.source, locked.source_type.as_str()) {
            (
                crate::config::SkillSource::GitHub {
                    path,
                    branch,
                    tag,
                    commit,
                    ..
                },
                "github",
            ) => {
                let repo_dir = tmp_root()?.join(&locked.name);
                if repo_dir.exists() {
                    let resolved = crate::registry::GitClient::resolve_local_head(&repo_dir)?;
                    locked.resolved_commit = resolved.commit.clone();
                    locked.resolved_tree_hash = crate::registry::GitClient::get_path_tree_hash(
                        &repo_dir,
                        path,
                    )?;
                    locked.resolved_reference = resolved.reference;
                    locked.resolved_at = Utc::now();
                }
                *branch = None;
                *tag = None;
                *commit = Some(locked.resolved_commit.clone());
            }
            (crate::config::SkillSource::Version(version), "version") => {
                *version = locked.resolved_commit.clone();
            }
            (crate::config::SkillSource::Local { path }, "local") => {
                let local_path = Path::new(path);
                if local_path.join(".git").exists() {
                    let resolved = crate::registry::GitClient::resolve_local_head(local_path)?;
                    locked.resolved_commit = resolved.commit;
                    locked.resolved_tree_hash = resolved.tree_hash;
                    locked.resolved_reference = resolved.reference;
                    locked.resolved_at = Utc::now();
                }
            }
            _ => {}
        }
    }

    config.save(&config_path)?;
    lockfile.save(&lockfile_path)?;
    println!("✓ Updated config from {}", lockfile_path.display());
    Ok(())
}

pub async fn remove(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, mut config, lockfile_path, lockfile) = config_and_lockfile()?;

    if config.skills.remove(&name).is_none() {
        return Err(format!("Skill '{}' not found", name).into());
    }

    config.save(&config_path)?;

    if let Some(mut lockfile) = lockfile {
        lockfile.remove_skill(&name);
        lockfile.save(&lockfile_path)?;
    }

    let tmp_dir = tmp_root()?.join(&name);
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir)?;
    }

    println!("✓ Removed '{}' from skills.toml", name);
    Ok(())
}

pub async fn info(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = config_and_lockfile()?;
    let skill = config
        .skills
        .get(&name)
        .ok_or_else(|| format!("Skill '{}' not found", name))?;

    println!("Skill: {}", name);
    println!("Source: {}", describe_skill_source(skill));
    println!(
        "Outdated: {}",
        format_outdated_state(classify_outdated(skill, lockfile.as_ref().and_then(|lock| lock.get_skill(&name))))
    );
    println!(
        "Lock: {}",
        describe_locked_skill(lockfile.as_ref().and_then(|lock| lock.get_skill(&name)))
    );

    Ok(())
}

pub async fn outdated() -> Result<(), Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = config_and_lockfile()?;

    if config.skills.is_empty() {
        println!("No skills configured.");
        return Ok(());
    }

    match lockfile {
        Some(lockfile) => {
            for (name, skill) in &config.skills {
                let state = classify_outdated(skill, lockfile.get_skill(name));
                println!("{}: {}", name, format_outdated_state(state));
            }
        }
        None => {
            for name in config.skills.keys() {
                println!("{}: missing-from-lock", name);
            }
        }
    }

    Ok(())
}

pub async fn doctor() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config, lockfile_path, lockfile) = config_and_lockfile()?;
    let store_path = crate::installer::ContentStore::default_path()?;
    let tmp_path = tmp_root()?;
    let mut pass_count = 0;
    let mut warn_count = 0;
    let mut fail_count = 0;

    println!("Configuration: {}", config_path.display());
    println!("Version: {}", config.version);
    println!("Skills configured: {}", config.skills.len());
    println!("Store path: {}", store_path.display());
    println!("Tmp path: {}", tmp_path.display());
    print_diagnostic(DiagnosticLevel::Pass, format!("store exists: {}", store_path.exists()));
    pass_count += 1;
    print_diagnostic(DiagnosticLevel::Pass, format!("tmp exists: {}", tmp_path.exists()));
    pass_count += 1;
    print_diagnostic(
        DiagnosticLevel::Pass,
        format!("lockfile exists: {}", lockfile_path.exists()),
    );
    pass_count += 1;

    if let Err(error) = config.validate() {
        print_diagnostic(DiagnosticLevel::Fail, format!("config validation: {}", error));
        fail_count += 1;
    } else {
        print_diagnostic(DiagnosticLevel::Pass, "config validation");
        pass_count += 1;
    }

    if let Some(lockfile) = lockfile.as_ref() {
        println!("Locked skills: {}", lockfile.skills.len());
        let configured: BTreeMap<_, _> = config.skills.keys().map(|name| (name.clone(), ())).collect();
        for locked in &lockfile.skills {
            if !configured.contains_key(&locked.name) {
                print_diagnostic(
                    DiagnosticLevel::Warn,
                    format!("locked skill '{}' missing from config", locked.name),
                );
                warn_count += 1;
            }

            if crate::installer::ContentStore::default()
                .get(&locked.resolved_tree_hash)
                .is_none()
            {
                print_diagnostic(
                    DiagnosticLevel::Warn,
                    format!(
                        "locked skill '{}' missing cached tree {}",
                        locked.name, locked.resolved_tree_hash
                    ),
                );
                warn_count += 1;
            }

            let tmp_repo = tmp_path.join(&locked.name);
            if locked.source_type == "github" && !tmp_repo.exists() {
                print_diagnostic(
                    DiagnosticLevel::Warn,
                    format!("github skill '{}' missing tmp clone {}", locked.name, tmp_repo.display()),
                );
                warn_count += 1;
            } else if locked.source_type == "github"
                && !crate::registry::GitClient::has_resolvable_head(&tmp_repo)
            {
                print_diagnostic(
                    DiagnosticLevel::Warn,
                    format!("github skill '{}' has broken tmp clone {}", locked.name, tmp_repo.display()),
                );
                warn_count += 1;
            }
        }
    }

    for (name, skill) in &config.skills {
        if let crate::config::SkillSource::Local { path } = &skill.source {
            if Path::new(path).exists() {
                print_diagnostic(
                    DiagnosticLevel::Pass,
                    format!("local skill '{}' path exists", name),
                );
                pass_count += 1;
            } else {
                print_diagnostic(
                    DiagnosticLevel::Warn,
                    format!("local skill '{}' path missing", name),
                );
                warn_count += 1;
            }
        }

        if let Some(lockfile_ref) = lockfile.as_ref() {
            for (level, message) in skill_health_lines(name, skill, Some(lockfile_ref), &tmp_path, &crate::installer::ContentStore::default()) {
                print_diagnostic(level, &message);
                match level {
                    DiagnosticLevel::Pass => pass_count += 1,
                    DiagnosticLevel::Warn => warn_count += 1,
                    DiagnosticLevel::Fail => fail_count += 1,
                }
            }
        }
    }

    println!(
        "Summary: {} pass, {} warn, {} fail",
        pass_count, warn_count, fail_count
    );

    if fail_count > 0 {
        return Err(format!("doctor found {} failing checks", fail_count).into());
    }

    Ok(())
}

pub async fn clean(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = data_root()?;
    let tmp_dir = data_dir.join("tmp");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir)?;
        println!("✓ Removed temporary directory {}", tmp_dir.display());
    }

    if all {
        let store_dir = data_dir.join("store");
        if store_dir.exists() {
            std::fs::remove_dir_all(&store_dir)?;
            println!("✓ Removed store directory {}", store_dir.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ConfigSkill, SkillSource};
    use git2::{Repository, Signature};
    use tempfile::TempDir;
    use tokio::sync::{Mutex, OnceCell};

    fn commit_file(repo_path: &Path, relative_path: &str, content: &str, message: &str) {
        std::fs::write(repo_path.join(relative_path), content).unwrap();

        let repo = Repository::open(repo_path).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new(relative_path)).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = Signature::now("skillmine", "skillmine@example.com").unwrap();

        let parent = repo
            .head()
            .ok()
            .and_then(|head| head.target())
            .and_then(|oid| repo.find_commit(oid).ok());

        match parent {
            Some(parent) => {
                repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[&parent],
                )
                .unwrap();
            }
            None => {
                repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])
                    .unwrap();
            }
        }
    }

    fn init_local_git_repo() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        Repository::init(temp_dir.path()).unwrap();
        commit_file(temp_dir.path(), "skill.md", "v1", "initial");
        temp_dir
    }

    async fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();
        LOCK.get_or_init(|| async { Mutex::new(()) }).await
    }

    #[tokio::test]
    async fn test_init_local_creates_config_file() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = init(true).await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(temp_dir.path().join("skills.toml").exists());
    }

    #[tokio::test]
    async fn test_add_updates_local_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let initial = Config::default();
        initial.save(&temp_dir.path().join("skills.toml")).unwrap();

        let result = add("anthropic/skills/git-release".to_string(), None, None).await;
        let updated = Config::load(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.contains_key("git-release"));
    }

    #[tokio::test]
    async fn test_install_with_empty_config_is_noop() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        Config::default()
            .save(&temp_dir.path().join("skills.toml"))
            .unwrap();

        let result = install(false, false).await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sync_local_skill_to_custom_target() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("local-skill");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("README.md"), "skill").unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-skill",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let target_dir = temp_dir.path().join("target-skills");
        let result = sync(
            "claude".to_string(),
            Some(target_dir.to_string_lossy().to_string()),
        )
        .await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(target_dir.join("local-skill").exists());
    }

    #[tokio::test]
    async fn test_list_and_remove_flow() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        assert!(list(false).await.is_ok());
        assert!(remove("demo".to_string()).await.is_ok());

        let updated = Config::load(&temp_dir.path().join("skills.toml")).unwrap();
        std::env::set_current_dir(original_dir).unwrap();

        assert!(updated.skills.is_empty());
    }

    #[tokio::test]
    async fn test_freeze_and_thaw_flow() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        assert!(freeze().await.is_ok());
        assert!(temp_dir.path().join(LOCKFILE_NAME).exists());
        assert!(thaw().await.is_ok());

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_clean_removes_tmp_directory() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let tmp_dir = temp_dir.path().join("skillmine").join("tmp");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let previous = dirs::data_local_dir();
        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }

        let result = clean(false).await;

        if let Some(value) = previous {
            unsafe {
                std::env::set_var("XDG_DATA_HOME", value);
            }
        }

        assert!(result.is_ok());
        assert!(!tmp_dir.exists());
    }

    #[tokio::test]
    async fn test_update_promotes_locked_commit_into_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: "abc123resolved".to_string(),
                resolved_tree_hash: "tree123".to_string(),
                resolved_reference: "branch:main".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let result = update(Some("demo".to_string())).await;
        let updated = Config::load(&temp_dir.path().join("skills.toml")).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        match &updated.skills["demo"].source {
            SkillSource::GitHub { branch, tag, commit, .. } => {
                assert!(branch.is_none());
                assert!(tag.is_none());
                assert_eq!(commit.as_deref(), Some("abc123resolved"));
            }
            _ => panic!("expected github source"),
        }
    }

    #[tokio::test]
    async fn test_outdated_detects_missing_lock_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let result = outdated().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_install_refreshes_lockfile_for_local_skill() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let source_dir = temp_dir.path().join("local-skill");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("README.md"), "skill").unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-skill",
            ConfigSkill {
                source: SkillSource::Local {
                    path: source_dir.to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let result = install(false, false).await;
        let lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(lockfile.get_skill("local-skill").is_some());
    }

    #[tokio::test]
    async fn test_doctor_runs_with_lock_mismatch() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "configured-only",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "locked-only".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^2.0".to_string()),
                resolved_commit: "^2.0".to_string(),
                resolved_tree_hash: "^2.0".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_outdated_detects_local_git_drift() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let initial_resolved = crate::registry::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "local-git".to_string(),
                source_type: "local".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: Some(local_repo.path().to_string_lossy().to_string()),
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(local_repo.path(), "skill.md", "v2", "second");

        let result = outdated().await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_refreshes_local_git_lock_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let initial_resolved = crate::registry::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let mut lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "local-git".to_string(),
                source_type: "local".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: Some(local_repo.path().to_string_lossy().to_string()),
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(local_repo.path(), "skill.md", "v2", "second");
        let latest_resolved = crate::registry::GitClient::resolve_local_head(local_repo.path()).unwrap();

        let result = update(Some("local-git".to_string())).await;
        lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let entry = lockfile.get_skill("local-git").unwrap();
        assert_eq!(entry.resolved_commit, latest_resolved.commit);
        assert_eq!(entry.resolved_tree_hash, latest_resolved.tree_hash);
    }

    #[tokio::test]
    async fn test_outdated_detects_github_tmp_clone_drift() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let repo_dir = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&repo_dir).unwrap();
        commit_file(&repo_dir, "skill.md", "v1", "initial");
        let initial_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(&repo_dir, "skill.md", "v2", "second");

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = outdated().await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_refreshes_github_tmp_clone_lock_entry() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let repo_dir = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&repo_dir).unwrap();
        commit_file(&repo_dir, "skill.md", "v1", "initial");
        let initial_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let mut lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(&repo_dir, "skill.md", "v2", "second");
        let latest_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = update(Some("demo".to_string())).await;
        lockfile = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
        let entry = lockfile.get_skill("demo").unwrap();
        assert_eq!(entry.resolved_commit, latest_resolved.commit);
        assert_eq!(entry.resolved_tree_hash, latest_resolved.tree_hash);
    }

    #[tokio::test]
    async fn test_install_prefers_locked_tree_hash_for_github_tmp_clone() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let store_root_dir = temp_dir.path().join("skillmine").join("store");
        let repo_dir = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&repo_dir).unwrap();
        commit_file(&repo_dir, "skill.md", "v1", "initial");
        let initial_resolved = crate::registry::GitClient::resolve_local_head(&repo_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: initial_resolved.commit.clone(),
                resolved_tree_hash: initial_resolved.tree_hash.clone(),
                resolved_reference: initial_resolved.reference.clone(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        commit_file(&repo_dir, "skill.md", "v2", "second");

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = install(false, false).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let refreshed_lock = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();
        let entry = refreshed_lock.get_skill("demo").unwrap();
        assert_eq!(entry.resolved_commit, initial_resolved.commit);
        assert!(store_root_dir.exists());
    }

    #[tokio::test]
    async fn test_sync_uses_locked_cached_content_for_github_skill() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let store = crate::installer::ContentStore::new(temp_dir.path().join("skillmine").join("store"));
        store.init().unwrap();
        let source_dir = temp_dir.path().join("cached-source");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("skill.md"), "cached").unwrap();
        let tree_hash = "ab1234567890";
        let stored_path = store.store(tree_hash, &source_dir).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: "commit123".to_string(),
                resolved_tree_hash: tree_hash.to_string(),
                resolved_reference: "branch:main".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let target_dir = temp_dir.path().join("target-skills");
        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = sync(
            "claude".to_string(),
            Some(target_dir.to_string_lossy().to_string()),
        )
        .await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        let synced = target_dir.join("demo");
        assert!(synced.exists());
        let canonical = std::fs::canonicalize(&synced).unwrap();
        assert_eq!(canonical, stored_path);
    }

    #[tokio::test]
    async fn test_doctor_fails_for_invalid_config() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        std::fs::write(
            temp_dir.path().join("skills.toml"),
            r#"version = "2.0"

[settings]
concurrency = 0
timeout = 300
auto_sync = false
"#,
        )
        .unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clean_all_removes_store_and_tmp() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let data_root = temp_dir.path().join("skillmine");
        let tmp_dir = data_root.join("tmp");
        let store_dir = data_root.join("store");
        std::fs::create_dir_all(&tmp_dir).unwrap();
        std::fs::create_dir_all(&store_dir).unwrap();
        std::fs::write(tmp_dir.join("temp.txt"), "tmp").unwrap();
        std::fs::write(store_dir.join("cache.txt"), "cache").unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = clean(true).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        assert!(result.is_ok());
        assert!(!tmp_dir.exists());
        assert!(!store_dir.exists());
    }

    #[tokio::test]
    async fn test_doctor_warns_when_locked_cache_missing_after_clean_all() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^1.0".to_string()),
                resolved_commit: "^1.0".to_string(),
                resolved_tree_hash: "missing-cache".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_doctor_reports_per_skill_status_lines() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let result = doctor().await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_outdated_marks_cache_missing_version_lock() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let state = classify_outdated(
            config.skills.get("demo").unwrap(),
            Some(&LockedSkill {
                name: "demo".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^1.0".to_string()),
                resolved_commit: "^1.0".to_string(),
                resolved_tree_hash: "missing-cache".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }),
        );

        std::env::set_current_dir(original_dir).unwrap();
        assert_eq!(state, OutdatedState::CacheMissing);
    }

    #[tokio::test]
    async fn test_classify_outdated_marks_pinned_github_skill() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: None,
                tag: None,
                commit: Some("abc123".to_string()),
            },
            name: None,
        };

        let state = classify_outdated(
            &skill,
            Some(&LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: Some("abc123".to_string()),
                local_path: None,
                version_constraint: None,
                resolved_commit: "abc123".to_string(),
                resolved_tree_hash: "tree123".to_string(),
                resolved_reference: "commit".to_string(),
                resolved_at: Utc::now(),
            }),
        );

        assert_eq!(state, OutdatedState::Pinned);
    }

    #[tokio::test]
    async fn test_classify_outdated_marks_tmp_missing_for_github_without_clone() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            },
            name: None,
        };

        let state = classify_outdated(
            &skill,
            Some(&LockedSkill {
                name: "demo".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: "abc123".to_string(),
                resolved_tree_hash: "tree123".to_string(),
                resolved_reference: "branch:main".to_string(),
                resolved_at: Utc::now(),
            }),
        );

        assert_eq!(state, OutdatedState::TmpMissing);
    }

    #[test]
    fn test_describe_skill_source_for_github_commit() {
        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: Some("sub/skill".to_string()),
                branch: None,
                tag: None,
                commit: Some("1234567890abcdef".to_string()),
            },
            name: None,
        };

        let rendered = describe_skill_source(&skill);
        assert!(rendered.contains("github:owner/repo"));
        assert!(rendered.contains("path=sub/skill"));
        assert!(rendered.contains("commit=12345678"));
    }

    #[test]
    fn test_describe_locked_skill_formats_short_hashes() {
        let locked = LockedSkill {
            name: "demo".to_string(),
            source_type: "github".to_string(),
            repo: Some("owner/repo".to_string()),
            path: None,
            requested_branch: Some("main".to_string()),
            requested_tag: None,
            requested_commit: None,
            local_path: None,
            version_constraint: None,
            resolved_commit: "abcdef1234567890".to_string(),
            resolved_tree_hash: "fedcba0987654321".to_string(),
            resolved_reference: "branch:main".to_string(),
            resolved_at: Utc::now(),
        };

        let rendered = describe_locked_skill(Some(&locked));
        assert!(rendered.contains("locked=abcdef12"));
        assert!(rendered.contains("tree=fedcba09"));
        assert!(rendered.contains("ref=branch:main"));
    }

    #[test]
    fn test_describe_locked_skill_none() {
        assert_eq!(describe_locked_skill(None), "locked=none");
    }

    #[test]
    fn test_diagnostic_summary_counts_levels() {
        let mut summary = DiagnosticSummary::default();
        summary.record(DiagnosticLevel::Pass);
        summary.record(DiagnosticLevel::Warn);
        summary.record(DiagnosticLevel::Fail);
        summary.record(DiagnosticLevel::Warn);

        assert_eq!(summary.pass, 1);
        assert_eq!(summary.warn, 2);
        assert_eq!(summary.fail, 1);
    }

    #[tokio::test]
    async fn test_mixed_multi_skill_workflow_regression() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let github_tmp = tmp_root_dir.join("github-skill");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&github_tmp).unwrap();
        commit_file(&github_tmp, "skill.md", "gh-v1", "initial");

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.add_skill(
            "github-skill",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: None,
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.add_skill(
            "version-skill",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        assert!(install(false, false).await.is_ok());
        assert!(freeze().await.is_ok());
        assert!(list(true).await.is_ok());
        assert!(outdated().await.is_ok());
        assert!(doctor().await.is_ok());
        assert!(clean(false).await.is_ok());
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert!(temp_dir.path().join(LOCKFILE_NAME).exists());
    }

    #[tokio::test]
    async fn test_outdated_does_not_mark_tmp_missing_when_valid_github_tmp_exists() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let github_tmp = tmp_root_dir.join("github-skill");
        std::fs::create_dir_all(&tmp_root_dir).unwrap();
        Repository::init(&github_tmp).unwrap();
        commit_file(&github_tmp, "skill.md", "gh-v1", "initial");
        let resolved = crate::registry::GitClient::resolve_local_head(&github_tmp).unwrap();

        let skill = ConfigSkill {
            source: SkillSource::GitHub {
                repo: "owner/repo".to_string(),
                path: None,
                branch: Some("main".to_string()),
                tag: None,
                commit: None,
            },
            name: None,
        };

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }

        let state = classify_outdated(
            &skill,
            Some(&LockedSkill {
                name: "github-skill".to_string(),
                source_type: "github".to_string(),
                repo: Some("owner/repo".to_string()),
                path: None,
                requested_branch: Some("main".to_string()),
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: None,
                resolved_commit: resolved.commit,
                resolved_tree_hash: resolved.tree_hash,
                resolved_reference: resolved.reference,
                resolved_at: Utc::now(),
            }),
        );

        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();
        assert_eq!(state, OutdatedState::UpToDate);
    }

    #[tokio::test]
    async fn test_end_to_end_local_lock_workflow() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let local_repo = init_local_git_repo();
        let target_dir = temp_dir.path().join("skills-target");

        let mut config = Config::default();
        config.add_skill(
            "local-git",
            ConfigSkill {
                source: SkillSource::Local {
                    path: local_repo.path().to_string_lossy().to_string(),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        assert!(install(false, false).await.is_ok());
        assert!(freeze().await.is_ok());
        commit_file(local_repo.path(), "skill.md", "v2", "second");
        assert!(outdated().await.is_ok());
        assert!(update(Some("local-git".to_string())).await.is_ok());
        assert!(thaw().await.is_ok());
        assert!(sync("claude".to_string(), Some(target_dir.to_string_lossy().to_string())).await.is_ok());
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(temp_dir.path().join(LOCKFILE_NAME).exists());
        assert!(target_dir.join("local-git").exists());
    }

    #[tokio::test]
    async fn test_remove_cleans_lock_and_tmp_clone() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tmp_root_dir = temp_dir.path().join("skillmine").join("tmp");
        let skill_tmp = tmp_root_dir.join("demo");
        std::fs::create_dir_all(&skill_tmp).unwrap();
        std::fs::write(skill_tmp.join("skill.md"), "temp").unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::Version("^1.0".to_string()),
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let lockfile = Lockfile {
            version: 1,
            generated_at: Utc::now(),
            config_path: "skills.toml".to_string(),
            skills: vec![LockedSkill {
                name: "demo".to_string(),
                source_type: "version".to_string(),
                repo: None,
                path: None,
                requested_branch: None,
                requested_tag: None,
                requested_commit: None,
                local_path: None,
                version_constraint: Some("^1.0".to_string()),
                resolved_commit: "^1.0".to_string(),
                resolved_tree_hash: "^1.0".to_string(),
                resolved_reference: "version".to_string(),
                resolved_at: Utc::now(),
            }],
        };
        lockfile.save(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = remove("demo".to_string()).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        let updated = Config::load(&temp_dir.path().join("skills.toml")).unwrap();
        let updated_lock = Lockfile::load(&temp_dir.path().join(LOCKFILE_NAME)).unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert!(updated.skills.is_empty());
        assert!(updated_lock.skills.is_empty());
        assert!(!skill_tmp.exists());
    }

    #[tokio::test]
    async fn test_list_should_not_mark_uninstalled_skill_as_installed_from_global_tmp() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let statuses = skill_statuses(
            "git-release",
            config.skills.get("git-release").unwrap(),
            None,
            &temp_dir.path().join("skillmine").join("tmp"),
            &crate::installer::ContentStore::new(temp_dir.path().join("skillmine").join("store")),
        );

        std::env::set_current_dir(original_dir).unwrap();

        assert_eq!(statuses, vec![SkillStatus::Configured]);
    }

    #[tokio::test]
    async fn test_freeze_fails_cleanly_for_uninstalled_github_skill() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = freeze().await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        let message = result.err().unwrap().to_string();
        assert!(message.contains("run install first") || message.contains("unresolved"));
    }

    #[tokio::test]
    async fn test_freeze_recovers_after_broken_tmp_git_repo_is_replaced() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let broken_tmp = temp_dir.path().join("skillmine").join("tmp").join("git-release");
        std::fs::create_dir_all(broken_tmp.join(".git/refs/heads")).unwrap();
        std::fs::write(broken_tmp.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(
            broken_tmp.join(".git/config"),
            "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
        )
        .unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: Some("1234567890abcdef1234567890abcdef12345678".to_string()),
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = freeze().await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert!(broken_tmp.exists());
    }

    #[tokio::test]
    async fn test_install_removes_broken_tmp_repo_before_reusing_it() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let broken_tmp = temp_dir.path().join("skillmine").join("tmp").join("git-release");
        std::fs::create_dir_all(broken_tmp.join(".git/refs/heads")).unwrap();
        std::fs::write(broken_tmp.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        std::fs::write(
            broken_tmp.join(".git/config"),
            "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
        )
        .unwrap();

        let mut config = Config::default();
        config.add_skill(
            "git-release",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "anthropic/skills".to_string(),
                    path: Some("git-release".to_string()),
                    branch: None,
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        unsafe {
            std::env::set_var("XDG_DATA_HOME", temp_dir.path());
        }
        let result = install(false, false).await;
        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err() || result.is_ok());
        assert!(!broken_tmp.exists());
    }

    #[tokio::test]
    async fn test_info_reports_skill_details() {
        let _guard = cwd_lock().await.lock().await;
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let mut config = Config::default();
        config.add_skill(
            "demo",
            ConfigSkill {
                source: SkillSource::GitHub {
                    repo: "owner/repo".to_string(),
                    path: Some("subdir".to_string()),
                    branch: Some("main".to_string()),
                    tag: None,
                    commit: None,
                },
                name: None,
            },
        );
        config.save(&temp_dir.path().join("skills.toml")).unwrap();

        let result = info("demo".to_string()).await;

        std::env::set_current_dir(original_dir).unwrap();
        assert!(result.is_ok());
    }
}
