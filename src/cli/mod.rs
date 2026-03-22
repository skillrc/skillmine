use chrono::Utc;
use std::path::{Path, PathBuf};
use crate::resolved_state::{lockfile_path_for, LockedSkill, Lockfile};
use crate::source_refs::version::resolve_version_source;

pub mod agent;
pub mod api;
pub mod bundle;
pub mod commands;
pub mod command;
pub mod create;
pub mod diagnostics;
pub mod instructions;
pub mod model;
pub mod pure;
pub mod state;
pub mod summary;
pub use create::create;
pub use summary::SkillSummary;
use pure::{describe_locked_skill, describe_skill_source};
use state::{classify_outdated, format_outdated_state, format_statuses, skill_statuses};
use summary::{apply_manifest_to_locked_skill, load_manifest_for_config_skill, skill_summary};

struct AddSkillInput {
    skill_name: String,
    skill: crate::config::ConfigSkill,
}

fn guard_add_skill_input(
    path: &str,
) -> Result<AddSkillInput, Box<dyn std::error::Error>> {
    use crate::config::{ConfigSkill, SkillSource};

    let candidate_path = PathBuf::from(path);
    if candidate_path.exists() {
        let normalized = candidate_path
            .canonicalize()
            .unwrap_or(candidate_path.clone());
        let path_string = normalized.to_string_lossy().to_string();
        let skill_name = SkillSource::Local {
            path: path_string.clone(),
        }
        .skill_name(path);

        return Ok(AddSkillInput {
            skill_name: skill_name.clone(),
            skill: ConfigSkill {
                source: SkillSource::Local { path: path_string },
                name: Some(skill_name),
                enabled: true,
                sync_enabled: true,
            },
        });
    }

    Err(crate::error::SkillmineError::Unsupported("Remote installation from GitHub is no longer supported. Please use local paths.".to_string()).into())
}

fn effect_write_default_config(config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = config_path.parent().ok_or("Invalid config path")?;
    std::fs::create_dir_all(config_dir)?;

    let default_config = r#"version = "1.0"

[settings]
# workspace = "~/Project/Skills"
auto_sync = false

[skills]
# Add your skills here
# Example:
# my-skill = { path = "~/Project/Skills/my-skill" }
"#;

    std::fs::write(config_path, default_config)?;
    Ok(())
}

fn emit_init_success(config_path: &Path) {
    println!("✓ Created configuration at {:?}", config_path);
    println!("\nNext steps:");
    println!("  1. Edit the file to add your local skills");
    println!("  2. Run 'skillmine install' to install them");
}

fn effect_add_skill_to_config(
    config_path: &Path,
    add_skill_input: AddSkillInput,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut config = crate::config::io::load_config(&config_path.to_path_buf())?;
    config.add_skill(&add_skill_input.skill_name, add_skill_input.skill);
    crate::config::io::save_config(&config, &config_path.to_path_buf())?;
    Ok(add_skill_input.skill_name)
}

fn emit_add_success(skill_name: &str) {
    println!("✓ Added '{}' to skills.toml", skill_name);
    println!("  Run 'skillmine install' to install it");
}

fn config_path_for_write() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(path) = crate::config::io::find_config() {
        return Ok(path);
    }

    Ok(dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("skillmine")
        .join("skills.toml"))
}

fn effect_set_config_value(
    config_path: &Path,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = if config_path.exists() {
        crate::config::io::load_config(&config_path.to_path_buf())?
    } else {
        crate::config::Config::default()
    };

    match key {
        "workspace" => config.settings.workspace = Some(value.to_string()),
        _ => return Err(format!("Unsupported config key '{}'. Only 'workspace' is supported.", key).into()),
    }

    let config_dir = config_path.parent().ok_or("Invalid config path")?;
    std::fs::create_dir_all(config_dir)?;
    crate::config::io::save_config(&config, &config_path.to_path_buf())?;
    Ok(())
}

fn format_config_show(config: &crate::config::Config, config_path: &Path) -> String {
    let workspace = config
        .settings
        .workspace
        .clone()
        .unwrap_or_else(|| "<unset>".to_string());

    format!(
        "Config: {}\nworkspace = {}",
        config_path.display(),
        workspace
    )
}


fn effect_set_skill_enabled(
    config_path: &Path,
    name: &str,
    enabled: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = crate::config::io::load_config(&config_path.to_path_buf())?;
    let skill = config
        .skills
        .get_mut(name)
        .ok_or_else(|| format!("Skill '{}' not found", name))?;
    skill.enabled = enabled;
    crate::config::io::save_config(&config, &config_path.to_path_buf())?;
    Ok(())
}

fn effect_set_skill_sync_enabled(
    config_path: &Path,
    name: &str,
    sync_enabled: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = crate::config::io::load_config(&config_path.to_path_buf())?;
    let skill = config
        .skills
        .get_mut(name)
        .ok_or_else(|| format!("Skill '{}' not found", name))?;
    skill.sync_enabled = sync_enabled;
    crate::config::io::save_config(&config, &config_path.to_path_buf())?;
    Ok(())
}

fn emit_set_skill_enabled(name: &str, enabled: bool) {
    let action = if enabled { "Enabled" } else { "Disabled" };
    println!("✓ {} '{}' in skills.toml", action, name);
}

fn emit_set_skill_sync_enabled(name: &str, sync_enabled: bool) {
    let action = if sync_enabled { "Resynced" } else { "Unsynced" };
    println!("✓ {} '{}' for runtime targets", action, name);
}

type ConfigBundle = (
    PathBuf,
    crate::config::Config,
    PathBuf,
    Option<Lockfile>,
);

pub(super) fn tmp_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(dirs::data_dir()
        .ok_or("Could not find data directory")?
        .join("skillmine")
        .join("tmp"))
}

pub(super) fn data_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(dirs::data_dir()
        .ok_or("Could not find data directory")?
        .join("skillmine"))
}

pub(super) fn config_and_lockfile() -> Result<ConfigBundle, Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;
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
    config: &crate::config::Config,
    tmp_root: &Path,
) -> Result<LockedSkill, Box<dyn std::error::Error>> {
    let resolved_skill = match resolve_version_source(name, skill, config) {
        Ok(skill) => skill,
        Err(crate::error::SkillmineError::Registry(_))
            if matches!(skill.source, crate::config::SkillSource::Version(_)) =>
        {
            skill.clone()
        }
        Err(error) => return Err(error.into()),
    };
    let version_constraint = match &skill.source {
        crate::config::SkillSource::Version(version) => Some(version.clone()),
        _ => None,
    };

    match &resolved_skill.source {
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
                    let tree_hash = crate::source_refs::GitClient::get_path_tree_hash(&repo_dir, path)?;
                    (commit.clone(), tree_hash, commit.clone())
                } else {
                    return Err(format!("Skill '{}' is pinned but not installed locally; run install first", name).into());
                }
            } else if repo_dir.exists() {
                let resolved = crate::source_refs::GitClient::resolve_source(&resolved_skill.source, &repo_dir)?;
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
                version_constraint,
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
                    let tree_hash = crate::source_refs::GitClient::get_path_tree_hash(&local_path, &None)?;
                    let resolved = crate::source_refs::GitClient::resolve_local_head(&local_path)?;
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
        if !skill.enabled {
            continue;
        }

        if let Some(existing) = previous_lockfile.and_then(|lock| lock.get_skill(name)) {
            if matches!(skill.source, crate::config::SkillSource::GitHub { .. }) {
                lockfile.skills.push(existing.clone());
                continue;
            }
        }

        if let Ok(mut locked_skill) = build_locked_skill(name, skill, config, &tmp_root) {
            if let Some(manifest) = load_manifest_for_config_skill(name, skill, &tmp_root) {
                apply_manifest_to_locked_skill(&mut locked_skill, &manifest);
            }
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

pub fn load_skill_summaries() -> Result<Vec<SkillSummary>, Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = config_and_lockfile()?;
    let tmp_root = tmp_root()?;
    let store = crate::installer::ContentStore::default();

    let mut summaries = Vec::new();
    for (name, skill) in &config.skills {
        summaries.push(skill_summary(name, skill, lockfile.as_ref(), &tmp_root, &store));
    }

    summaries.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(summaries)
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

    effect_write_default_config(&config_path)?;
    emit_init_success(&config_path);

    Ok(())
}


pub async fn add(
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    add_with_options(path, true).await.map(|_| ())
}

async fn add_with_config_path(
    config_path: PathBuf,
    path: String,
    emit_output: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let add_skill_input = guard_add_skill_input(&path)?;
    let skill_name = effect_add_skill_to_config(&config_path, add_skill_input)?;
    if emit_output {
        emit_add_success(&skill_name);
    }

    Ok(format!(
        "Added local source '{}' to configuration. Next: install to prepare it locally.",
        path
    ))
}

pub async fn add_with_options(
    path: String,
    emit_output: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    add_with_config_path(config_path, path, emit_output).await
}

pub async fn config_set(
    key: String,
    value: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let config_path = config_path_for_write()?;
    effect_set_config_value(&config_path, &key, &value)?;
    Ok(format!(
        "Updated {} in {}",
        key,
        config_path.display()
    ))
}

pub async fn config_show() -> Result<String, Box<dyn std::error::Error>> {
    let config_path = config_path_for_write()?;
    let config = if config_path.exists() {
        crate::config::io::load_config(&config_path)?
    } else {
        crate::config::Config::default()
    };

    Ok(format_config_show(&config, &config_path))
}

pub async fn create_and_add(
    name: String,
    output_dir: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_local_config()?;
    let created = create::create_created_skill(name, output_dir)?;
    let add_message = add_with_config_path(
        config_path,
        created.target_dir.to_string_lossy().to_string(),
        false,
    )
    .await?;

    Ok(format!("{}\n{}", created.message, add_message))
}

pub async fn enable(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    effect_set_skill_enabled(&config_path, &name, true)?;
    emit_set_skill_enabled(&name, true);
    Ok(())
}

pub async fn disable(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    effect_set_skill_enabled(&config_path, &name, false)?;
    emit_set_skill_enabled(&name, false);
    Ok(())
}

pub async fn unsync(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    effect_set_skill_sync_enabled(&config_path, &name, false)?;
    emit_set_skill_sync_enabled(&name, false);
    Ok(())
}

pub async fn resync(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = crate::config::io::find_config()?;
    effect_set_skill_sync_enabled(&config_path, &name, true)?;
    emit_set_skill_sync_enabled(&name, true);
    Ok(())
}

pub async fn install(force: bool, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::installer::{ContentStore, InstallContext, InstallOutcomeKind, InstallSummary};

    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;
    let lockfile_path = lockfile_path_for(&config_path);
    let existing_lockfile = if lockfile_path.exists() {
        Some(Lockfile::load(&lockfile_path)?)
    } else {
        None
    };

    if config.skills.is_empty() {
        println!("No skill sources configured. Run 'skillmine add <source>' to add one.");
        return Ok(());
    }

    let store = ContentStore::default();
    store.init()?;

    let install_dir = tmp_root()?;
    std::fs::create_dir_all(&install_dir)?;
    let install_context = InstallContext {
        install_dir: install_dir.clone(),
        force,
        verbose,
    };

    let outcomes = crate::installer::install_many_skills(
        config
            .skills
            .iter()
            .filter(|(_, skill)| skill.enabled)
            .map(|(name, skill)| (name.clone(), skill.clone()))
            .collect(),
        config.clone(),
        existing_lockfile.clone(),
        store.clone(),
        install_context,
        config.skills.len().max(1),
    )
    .await;

    let mut summary = InstallSummary::default();

    for outcome in &outcomes {
        let name = &outcome.name;
        if let Some(message) = &outcome.message {
            match outcome.kind {
                InstallOutcomeKind::Installed => {
                    if verbose {
                        println!("  ✓ {}", message);
                    }
                }
                InstallOutcomeKind::Skipped => {
                    eprintln!("Skipping '{}': {}", name, message);
                }
                InstallOutcomeKind::Error => {
                    eprintln!("{}", message);
                }
            }
        }
        summary.record(outcome);
    }

    println!("\nInstallation complete:");
    println!("  {} installed", summary.installed);
    println!("  {} skipped", summary.skipped);
    if summary.errors > 0 {
        println!("  {} errors", summary.errors);
    }

    refresh_lockfile_from_current_state(&config_path, &config, existing_lockfile.as_ref())?;

    Ok(())
}

pub async fn install_selected(name: Option<String>, force: bool, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if name.is_none() {
        return install(force, verbose).await;
    }

    use crate::installer::{ContentStore, InstallContext, InstallOutcomeKind};

    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;
    let lockfile_path = lockfile_path_for(&config_path);
    let existing_lockfile = if lockfile_path.exists() {
        Some(Lockfile::load(&lockfile_path)?)
    } else {
        None
    };

    let selected = name.unwrap();
    let Some(skill) = config.skills.get(&selected) else {
        return Err(format!("Skill '{}' not found", selected).into());
    };

    if !skill.enabled {
        println!("Skipping '{}': disabled", selected);
        return Ok(());
    }

    let store = ContentStore::default();
    store.init()?;

    let install_dir = tmp_root()?;
    std::fs::create_dir_all(&install_dir)?;
    let install_context = InstallContext {
        install_dir,
        force,
        verbose,
    };

    let outcome = crate::installer::install_skill_to_store(
        &selected,
        skill,
        &config,
        existing_lockfile.as_ref(),
        &store,
        &install_context,
    );

    match outcome.kind {
        InstallOutcomeKind::Installed => {}
        InstallOutcomeKind::Skipped | InstallOutcomeKind::Error => {
            return Err(outcome
                .message
                .unwrap_or_else(|| format!("Failed to install '{}'", selected))
                .into())
        }
    }

    refresh_lockfile_from_current_state(&config_path, &config, existing_lockfile.as_ref())?;
    if verbose {
        println!("✓ Installed '{}'", selected);
    }
    Ok(())
}

pub async fn sync(target: String, path: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    sync_with_options(target, path, true).await
}

pub async fn sync_with_options(
    target: String,
    path: Option<String>,
    emit_output: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    use crate::installer::ContentStore;
    use crate::source_refs::GitClient;
    use std::os::unix::fs::symlink;

    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;
    let lockfile_path = lockfile_path_for(&config_path);
    let lockfile = if lockfile_path.exists() {
        Some(Lockfile::load(&lockfile_path)?)
    } else {
        None
    };

    if config.skills.is_empty() {
        if emit_output {
            println!("No skills configured.");
        }
        return Ok("No skills configured.".to_string());
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
    let mut report_lines = Vec::new();

    for (name, skill) in &config.skills {
        if !skill.enabled {
            if emit_output {
                println!("- Skipping '{}' (disabled)", name);
            }
            report_lines.push(format!("- Skipping '{}' (disabled)", name));
            continue;
        }
        if !skill.sync_enabled {
            if emit_output {
                println!("- Skipping '{}' (unsynced)", name);
            }
            report_lines.push(format!("- Skipping '{}' (unsynced)", name));
            continue;
        }

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
                        if emit_output {
                            eprintln!("Failed to symlink '{}': {}", name, error);
                        }
                        report_lines.push(format!("✗ Failed '{}' -> {}", name, error));
                        errors += 1;
                    } else {
                        if emit_output {
                            println!("✓ Synced '{}' -> {}", name, skill_target.display());
                        }
                        report_lines.push(format!("✓ Synced '{}' -> {}", name, skill_target.display()));
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
                                    if emit_output {
                                        eprintln!("Failed to symlink '{}': {}", name, error);
                                    }
                                    report_lines.push(format!("✗ Failed '{}' -> {}", name, error));
                                    errors += 1;
                                } else {
                                    if emit_output {
                                        println!("✓ Synced '{}' -> {}", name, skill_target.display());
                                    }
                                    report_lines.push(format!("✓ Synced '{}' -> {}", name, skill_target.display()));
                                    synced += 1;
                                }
                            } else {
                                if emit_output {
                                    eprintln!("Failed to get source for '{}'", name);
                                }
                                report_lines.push(format!("✗ Failed to get source for '{}'", name));
                                errors += 1;
                            }
                        }
                        Err(error) => {
                            if emit_output {
                                eprintln!("Failed to resolve '{}': {}", name, error);
                            }
                            report_lines.push(format!("✗ Failed to resolve '{}': {}", name, error));
                            errors += 1;
                        }
                    }
                }
            }
            crate::config::SkillSource::Local { path: local_path } => {
                let source = std::path::PathBuf::from(local_path);
                if source.exists() {
                    if let Err(error) = symlink(&source, &skill_target) {
                        if emit_output {
                            eprintln!("Failed to symlink '{}': {}", name, error);
                        }
                        report_lines.push(format!("✗ Failed '{}' -> {}", name, error));
                        errors += 1;
                    } else {
                        if emit_output {
                            println!("✓ Synced '{}' -> {}", name, skill_target.display());
                        }
                        report_lines.push(format!("✓ Synced '{}' -> {}", name, skill_target.display()));
                        synced += 1;
                    }
                } else {
                    if emit_output {
                        eprintln!("Local path for '{}' does not exist: {}", name, source.display());
                    }
                    report_lines.push(format!("✗ Local path missing for '{}': {}", name, source.display()));
                    errors += 1;
                }
            }
            _ => {
                if emit_output {
                    eprintln!("Skipping '{}': unsupported source type", name);
                }
                report_lines.push(format!("- Skipping '{}' (unsupported source type)", name));
            }
        }
    }

    if emit_output {
        println!("\nSync complete:");
        println!("  {} synced to {}", synced, target_dir.display());
        if errors > 0 {
            println!("  {} errors", errors);
        }
    }

    report_lines.push(String::new());
    report_lines.push("Sync complete:".to_string());
    report_lines.push(format!("  {} synced to {}", synced, target_dir.display()));
    if errors > 0 {
        report_lines.push(format!("  {} errors", errors));
    }

    Ok(report_lines.join("\n"))
}

pub async fn freeze() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config, lockfile_path, _) = config_and_lockfile()?;
    let tmp_root = tmp_root()?;
    let mut lockfile = Lockfile::new(&config_path);

    for (name, skill) in &config.skills {
        if !skill.enabled {
            continue;
        }

        lockfile
            .skills
            .push(build_locked_skill(name, skill, &config, &tmp_root)?);
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

    crate::config::io::save_config(&config, &config_path)?;
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
            let summary = skill_summary(name, skill, lockfile.as_ref(), &tmp_root, &store);
            println!("- {} [{}]", name, format_statuses(&statuses));
            println!("  source: {}", describe_skill_source(skill));
            println!("  outdated: {}", outdated_state);
            println!("  {}", locked_summary);
            if let Some(version) = summary.skill_version {
                println!("  manifest version: {}", version);
            }
            if let Some(maturity) = summary.maturity {
                println!("  maturity: {}", maturity);
            }
            if let Some(last_verified) = summary.last_verified {
                println!("  last verified: {}", last_verified);
            }
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
                    let resolved = crate::source_refs::GitClient::resolve_local_head(&repo_dir)?;
                    locked.resolved_commit = resolved.commit.clone();
                    locked.resolved_tree_hash = crate::source_refs::GitClient::get_path_tree_hash(
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
                    let resolved = crate::source_refs::GitClient::resolve_local_head(local_path)?;
                    locked.resolved_commit = resolved.commit;
                    locked.resolved_tree_hash = resolved.tree_hash;
                    locked.resolved_reference = resolved.reference;
                    locked.resolved_at = Utc::now();
                }
            }
            _ => {}
        }
    }

    crate::config::io::save_config(&config, &config_path)?;
    lockfile.save(&lockfile_path)?;
    println!("✓ Updated config from {}", lockfile_path.display());
    Ok(())
}

pub async fn remove(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, mut config, lockfile_path, lockfile) = config_and_lockfile()?;

    if config.skills.remove(&name).is_none() {
        return Err(format!("Skill '{}' not found", name).into());
    }

    crate::config::io::save_config(&config, &config_path)?;

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

#[cfg(test)]
mod tests;
