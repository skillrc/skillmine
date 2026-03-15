use std::path::{Path, PathBuf};

use crate::config::ConfigSkill;
use crate::installer::ContentStore;
use crate::lockfile::{LockedSkill, Lockfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillStatus {
    Configured,
    Installed,
    Cached,
    Locked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutdatedState {
    UpToDate,
    LockDrift,
    MissingFromLock,
    Local,
    CacheMissing,
    TmpMissing,
    Pinned,
    SourceMismatch,
}

pub fn broken_tmp_repo_path(skill: &ConfigSkill, tmp_root: &Path, name: &str) -> Option<PathBuf> {
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

pub fn skill_statuses(
    name: &str,
    skill: &ConfigSkill,
    lockfile: Option<&Lockfile>,
    tmp_root: &Path,
    store: &ContentStore,
) -> Vec<SkillStatus> {
    let mut statuses = vec![SkillStatus::Configured];
    let locked_skill = lockfile.and_then(|lock| lock.get_skill(name));

    let installed_path = match &skill.source {
        crate::config::SkillSource::GitHub {
            path: Some(subpath),
            ..
        } => tmp_root.join(name).join(subpath),
        crate::config::SkillSource::GitHub { .. } => tmp_root.join(name),
        crate::config::SkillSource::Local { path } => PathBuf::from(path),
        crate::config::SkillSource::Version(_) => PathBuf::new(),
    };

    match &skill.source {
        crate::config::SkillSource::GitHub { .. } => {
            if locked_skill.is_some()
                && !installed_path.as_os_str().is_empty()
                && installed_path.exists()
            {
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

pub fn format_statuses(statuses: &[SkillStatus]) -> String {
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

pub fn classify_outdated(skill: &ConfigSkill, locked: Option<&LockedSkill>) -> OutdatedState {
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

                if !tmp_repo.exists() || !crate::registry::GitClient::has_resolvable_head(&tmp_repo)
                {
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

pub fn format_outdated_state(state: OutdatedState) -> &'static str {
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
