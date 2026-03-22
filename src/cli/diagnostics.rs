use std::path::Path;

use crate::config::ConfigSkill;
use crate::installer::ContentStore;
use crate::resolved_state::Lockfile;
use crate::source_refs::GitClient;

use super::state::{classify_outdated, format_outdated_state, OutdatedState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Pass,
    Warn,
    Fail,
}

impl DiagnosticLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warn => "WARN",
            Self::Fail => "FAIL",
        }
    }
}

pub fn print_diagnostic(level: DiagnosticLevel, message: impl AsRef<str>) {
    println!("{}: {}", level.as_str(), message.as_ref());
}

pub fn skill_health_lines(
    name: &str,
    skill: &ConfigSkill,
    lockfile: Option<&Lockfile>,
    tmp_path: &Path,
    store: &ContentStore,
) -> Vec<(DiagnosticLevel, String)> {
    let mut lines = Vec::new();

    let state = classify_outdated(
        skill,
        lockfile.and_then(|lock: &Lockfile| lock.get_skill(name)),
    );
    lines.push((
        match state {
            OutdatedState::LockDrift | OutdatedState::SourceMismatch => DiagnosticLevel::Warn,
            OutdatedState::CacheMissing | OutdatedState::TmpMissing => DiagnosticLevel::Warn,
            OutdatedState::UpToDate
            | OutdatedState::Pinned
            | OutdatedState::Local
            | OutdatedState::MissingFromLock => DiagnosticLevel::Pass,
        },
        format!("skill '{}' state: {}", name, format_outdated_state(state)),
    ));

    if let Some(locked) = lockfile.and_then(|lock: &Lockfile| lock.get_skill(name)) {
        if store.get(&locked.resolved_tree_hash).is_some() {
            lines.push((
                DiagnosticLevel::Pass,
                format!("skill '{}' prepared content present", name),
            ));
        }
    }

    if matches!(skill.source, crate::config::SkillSource::GitHub { .. }) {
        let repo_dir = tmp_path.join(name);
        if repo_dir.exists() && GitClient::has_resolvable_head(&repo_dir) {
            lines.push((
                DiagnosticLevel::Pass,
                format!("skill '{}' local checkout healthy", name),
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
pub struct DiagnosticSummary {
    pub pass: usize,
    pub warn: usize,
    pub fail: usize,
}

impl DiagnosticSummary {
    #[allow(dead_code)]
    pub fn record(&mut self, level: DiagnosticLevel) {
        match level {
            DiagnosticLevel::Pass => self.pass += 1,
            DiagnosticLevel::Warn => self.warn += 1,
            DiagnosticLevel::Fail => self.fail += 1,
        }
    }
}
