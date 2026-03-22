#![allow(dead_code)]
use crate::error::{Result, SkillmineError};
use crate::config::Config;
use std::fs;
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use crate::config::SkillSource;

use super::effect::symlink_dir_all;

// ============================================================================
// Waterflow Architecture: Sync Plan for Local Assets
// ============================================================================

/// Asset types supported for sync
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    Skill,
    Command,
    Agent,
}

impl AssetKind {
    pub fn target_subdir(&self) -> &'static str {
        match self {
            AssetKind::Skill => "skills",
            AssetKind::Command => "commands",
            AssetKind::Agent => "agents",
        }
    }

    pub fn source_filename(&self) -> &'static str {
        match self {
            AssetKind::Skill => "SKILL.md",
            AssetKind::Command => "COMMAND.md",
            AssetKind::Agent => "AGENT.md",
        }
    }
}

/// A single sync operation
#[derive(Debug, Clone)]
pub struct SyncOp {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub asset_name: String,
    pub asset_kind: AssetKind,
}

/// Complete sync plan for multiple assets
#[derive(Debug, Clone)]
pub struct SyncPlan {
    pub ops: Vec<SyncOp>,
    pub target_runtime: RuntimeTarget,
}

/// Runtime target for sync
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTarget {
    OpenCode,
    Claude,
}

impl RuntimeTarget {
    pub fn target_dir(&self) -> PathBuf {
        match self {
            RuntimeTarget::OpenCode => dirs::config_dir()
                .expect("Failed to get config dir")
                .join("opencode"),
            RuntimeTarget::Claude => dirs::home_dir()
                .expect("Failed to get home dir")
                .join(".claude"),
        }
    }
}

// ============================================================================
// GUARD: Input validation
// ============================================================================

fn guard_source_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(SkillmineError::Installation(
            format!("Source path does not exist: {}", path.display())
        ));
    }
    Ok(())
}

fn guard_asset_file_exists(source_dir: &Path, asset_kind: AssetKind) -> Result<()> {
    let asset_file = source_dir.join(asset_kind.source_filename());
    if !asset_file.exists() {
        return Err(SkillmineError::Installation(
            format!("Required file not found: {}", asset_file.display())
        ));
    }
    Ok(())
}

// ============================================================================
// SHAPE: Path construction
// ============================================================================

fn shape_source_path(config_path: &str, base_dir: Option<&Path>) -> PathBuf {
    let path = PathBuf::from(config_path);
    if path.is_absolute() {
        path
    } else if let Some(base) = base_dir {
        base.join(path)
    } else {
        path
    }
}

fn shape_target_path(
    asset_name: &str,
    asset_kind: AssetKind,
    target: RuntimeTarget,
) -> PathBuf {
    let target_base = target.target_dir();
    match asset_kind {
        AssetKind::Skill => {
            // Skills go to: ~/.config/opencode/skills/{name}/SKILL.md
            target_base.join(asset_kind.target_subdir()).join(asset_name)
        }
        AssetKind::Command | AssetKind::Agent => {
            // Commands/Agents go to: ~/.config/opencode/{commands,agents}/{name}.md
            target_base
                .join(asset_kind.target_subdir())
                .join(format!("{}.md", asset_name))
        }
    }
}

// ============================================================================
// TRANSFORM: Build sync plans
// ============================================================================

/// Transform skill config into sync plan
pub fn transform_skill_sync_plan(
    name: &str,
    source_path: &Path,
    target: RuntimeTarget,
) -> Result<SyncOp> {
    guard_source_exists(source_path)?;
    guard_asset_file_exists(source_path, AssetKind::Skill)?;

    let target_path = shape_target_path(name, AssetKind::Skill, target);

    Ok(SyncOp {
        source_path: source_path.to_path_buf(),
        target_path,
        asset_name: name.to_string(),
        asset_kind: AssetKind::Skill,
    })
}

/// Transform all enabled skills from config into sync plan
pub fn transform_config_sync_plan(
    config: &Config,
    target: RuntimeTarget,
) -> Result<SyncPlan> {
    let mut ops = Vec::new();

    for (name, skill) in &config.skills {
        if !skill.enabled {
            continue;
        }

        // Only process local skills
        if let crate::config::SkillSource::Local { path } = &skill.source {
            let source_path = shape_source_path(path, None);

            match transform_skill_sync_plan(name, &source_path, target) {
                Ok(op) => ops.push(op),
                Err(e) => eprintln!("Warning: Skipping skill '{}': {}", name, e),
            }
        }
    }

    Ok(SyncPlan { ops, target_runtime: target })
}

// ============================================================================
// EFFECT: Execute sync operations
// ============================================================================

pub fn effect_execute_sync_plan(plan: &SyncPlan) -> Result<Vec<SyncOutcome>> {
    let mut outcomes = Vec::new();

    for op in &plan.ops {
        let outcome = effect_execute_sync_op(op)?;
        outcomes.push(outcome);
    }

    Ok(outcomes)
}

fn effect_execute_sync_op(op: &SyncOp) -> Result<SyncOutcome> {
    // Ensure parent directory exists
    if let Some(parent) = op.target_path.parent() {
        fs::create_dir_all(parent).map_err(SkillmineError::Io)?;
    }

    // Remove existing broken symlink
    if op.target_path.is_symlink() && !op.target_path.exists() {
        fs::remove_file(&op.target_path).map_err(SkillmineError::Io)?;
    }

    match op.asset_kind {
        AssetKind::Skill => {
            // For skills, symlink the entire directory
            symlink_dir_all(&op.source_path, &op.target_path)?;
        }
        AssetKind::Command | AssetKind::Agent => {
            // For commands/agents, symlink the single markdown file
            #[cfg(unix)]
            {
                use std::os::unix::fs::symlink;
                let source_file = op.source_path.join(op.asset_kind.source_filename());
                symlink(&source_file, &op.target_path).map_err(SkillmineError::Io)?;
            }
            #[cfg(windows)]
            {
                use std::os::windows::fs::symlink_file;
                let source_file = op.source_path.join(op.asset_kind.source_filename());
                symlink_file(&source_file, &op.target_path).map_err(SkillmineError::Io)?;
            }
        }
    }

    Ok(SyncOutcome {
        asset_name: op.asset_name.clone(),
        asset_kind: op.asset_kind,
        target_path: op.target_path.clone(),
        success: true,
        message: None,
    })
}

// ============================================================================
// EMIT: Output formatting
// ============================================================================

#[derive(Debug, Clone)]
pub struct SyncOutcome {
    pub asset_name: String,
    pub asset_kind: AssetKind,
    pub target_path: PathBuf,
    pub success: bool,
    pub message: Option<String>,
}

pub fn emit_sync_summary(outcomes: &[SyncOutcome]) -> String {
    let success_count = outcomes.iter().filter(|o| o.success).count();
    let total = outcomes.len();

    let mut output = format!("Synced {}/{} assets\n", success_count, total);

    for outcome in outcomes {
        let status = if outcome.success { "✓" } else { "✗" };
        let kind = format!("{:?}", outcome.asset_kind).to_lowercase();
        output.push_str(&format!(
            "  {} {} '{}' -> {}\n",
            status,
            kind,
            outcome.asset_name,
            outcome.target_path.display()
        ));
    }

    output
}

// ============================================================================
// OBSERVE: Logging
// ============================================================================

fn observe_sync_start(plan: &SyncPlan) {
    #[cfg(debug_assertions)]
    eprintln!(
        "[OBSERVE] Starting sync to {:?}, {} operations",
        plan.target_runtime,
        plan.ops.len()
    );
}

fn observe_sync_complete(outcomes: &[SyncOutcome]) {
    #[cfg(debug_assertions)]
    eprintln!("[OBSERVE] Sync completed, {} outcomes", outcomes.len());
}

// ============================================================================
// PUBLIC API: Complete sync flow
// ============================================================================

/// Full Waterflow: Transform -> Effect -> Emit for skill sync
pub fn sync_skills(
    config: &Config,
    target: RuntimeTarget,
) -> Result<String> {
    // Transform
    let plan = transform_config_sync_plan(config, target)?;

    if plan.ops.is_empty() {
        return Ok("No enabled skills to sync".to_string());
    }

    observe_sync_start(&plan);

    // Effect
    let outcomes = effect_execute_sync_plan(&plan)?;

    observe_sync_complete(&outcomes);

    // Emit
    Ok(emit_sync_summary(&outcomes))
}

/// Sync a single local skill (for create-and-sync workflow)
pub fn sync_single_skill(
    name: &str,
    source_path: &Path,
    target: RuntimeTarget,
) -> Result<String> {
    // Transform
    let op = transform_skill_sync_plan(name, source_path, target)?;
    let plan = SyncPlan {
        ops: vec![op],
        target_runtime: target,
    };

    observe_sync_start(&plan);

    // Effect
    let outcomes = effect_execute_sync_plan(&plan)?;

    observe_sync_complete(&outcomes);

    // Emit
    Ok(emit_sync_summary(&outcomes))
}

// ============================================================================
// COMPAT: Stubs for legacy code that still references old installer API
// These are no-op implementations since we moved to local-first symlink sync.
// ============================================================================

/// Legacy content-addressed store stub (no-op for local-first mode)
#[derive(Clone, Default)]
pub struct ContentStore;

impl ContentStore {
    pub fn new(_path: PathBuf) -> Self { Self }
    pub fn init(&self) -> Result<()> { Ok(()) }
    pub fn get(&self, _hash: &str) -> Option<PathBuf> { None }
    #[allow(dead_code)]
    pub fn store(&self, _hash: &str, _source: &Path) -> Result<PathBuf> {
        Err(SkillmineError::Installation("ContentStore disabled in local-first mode".to_string()))
    }
    pub fn default_path() -> Result<PathBuf> {
        Ok(dirs::data_dir()
            .ok_or_else(|| SkillmineError::Installation("Could not find data directory".to_string()))?
            .join("skillmine")
            .join("store"))
    }
}

/// Legacy install context stub
#[derive(Clone)]
pub struct InstallContext {
    pub install_dir: PathBuf,
    pub force: bool,
    pub verbose: bool,
}

/// Legacy install outcome kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallOutcomeKind {
    Installed,
    Skipped,
    Error,
}

/// Legacy install outcome
#[derive(Debug, Clone)]
pub struct InstallOutcome {
    pub name: String,
    pub kind: InstallOutcomeKind,
    pub message: Option<String>,
}

/// Legacy install summary
#[derive(Debug, Clone, Default)]
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

/// Legacy: install many skills (now redirects to sync_skills)
pub async fn install_many_skills(
    skills: Vec<(String, crate::config::ConfigSkill)>,
    _config: crate::config::Config,
    _lockfile: Option<crate::resolved_state::Lockfile>,
    _store: ContentStore,
    _context: InstallContext,
    _concurrency: usize,
) -> Vec<InstallOutcome> {
    let mut outcomes = Vec::new();
    for (name, skill) in &skills {
        if let crate::config::SkillSource::Local { path } = &skill.source {
            let source = PathBuf::from(path);
            if source.exists() {
                outcomes.push(InstallOutcome {
                    name: name.clone(),
                    kind: InstallOutcomeKind::Installed,
                    message: Some(format!("Local skill '{}' ready at {}", name, source.display())),
                });
            } else {
                outcomes.push(InstallOutcome {
                    name: name.clone(),
                    kind: InstallOutcomeKind::Error,
                    message: Some(format!("Local path does not exist: {}", source.display())),
                });
            }
        } else {
            outcomes.push(InstallOutcome {
                name: name.clone(),
                kind: InstallOutcomeKind::Skipped,
                message: Some("Remote skills are no longer supported; use local paths".to_string()),
            });
        }
    }
    outcomes
}

/// Legacy: install a single skill to store
pub fn install_skill_to_store(
    name: &str,
    skill: &crate::config::ConfigSkill,
    _config: &crate::config::Config,
    _lockfile: Option<&crate::resolved_state::Lockfile>,
    _store: &ContentStore,
    _context: &InstallContext,
) -> InstallOutcome {
    if let crate::config::SkillSource::Local { path } = &skill.source {
        let source = PathBuf::from(path);
        if source.exists() {
            InstallOutcome {
                name: name.to_string(),
                kind: InstallOutcomeKind::Installed,
                message: Some(format!("Local skill '{}' ready", name)),
            }
        } else {
            InstallOutcome {
                name: name.to_string(),
                kind: InstallOutcomeKind::Error,
                message: Some(format!("Local path does not exist: {}", source.display())),
            }
        }
    } else {
        InstallOutcome {
            name: name.to_string(),
            kind: InstallOutcomeKind::Skipped,
            message: Some("Remote skills are no longer supported; use local paths".to_string()),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_asset_kind_target_subdir() {
        assert_eq!(AssetKind::Skill.target_subdir(), "skills");
        assert_eq!(AssetKind::Command.target_subdir(), "commands");
        assert_eq!(AssetKind::Agent.target_subdir(), "agents");
    }

    #[test]
    fn test_shape_target_path_skill() {
        let path = shape_target_path("my-skill", AssetKind::Skill, RuntimeTarget::OpenCode);
        assert!(path.to_string_lossy().contains("skills/my-skill"));
    }

    #[test]
    fn test_shape_target_path_command() {
        let path = shape_target_path("my-cmd", AssetKind::Command, RuntimeTarget::OpenCode);
        assert!(path.to_string_lossy().contains("commands/my-cmd.md"));
    }

    #[test]
    fn test_guard_source_exists() {
        let temp = TempDir::new().unwrap();
        let existing = temp.path().join("exists");
        fs::create_dir(&existing).unwrap();

        assert!(guard_source_exists(&existing).is_ok());
        assert!(guard_source_exists(&temp.path().join("not-exists")).is_err());
    }
}
