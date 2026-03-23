use std::path::Path;

use crate::config::ConfigSkill;
use crate::installer::ContentStore;
use crate::resolved_state::{LockedSkill, Lockfile};

use super::pure::{describe_locked_skill, describe_skill_source};
use super::state::{classify_outdated, format_statuses, skill_statuses};

#[derive(Debug, Clone)]
pub struct SkillSummary {
    pub name: String,
    pub asset_type: String,
    pub source: String,
    pub enabled: bool,
    pub statuses: Vec<String>,
    pub outdated: String,
    pub lock_summary: String,
    pub manifest_version: Option<String>,
    pub skill_version: Option<String>,
    pub maturity: Option<String>,
    pub last_verified: Option<String>,
    pub description: Option<String>,
}

pub fn load_manifest_for_config_skill(
    name: &str,
    skill: &ConfigSkill,
    tmp_root: &Path,
) -> Option<crate::manifest::SkillManifest> {
    match &skill.source {
        crate::config::SkillSource::GitHub { path, .. } => {
            crate::manifest::load_manifest(&tmp_root.join(name), path)
                .ok()
                .flatten()
        }
        crate::config::SkillSource::Local { path } => {
            crate::manifest::load_manifest(Path::new(path), &None)
                .ok()
                .flatten()
        }
        crate::config::SkillSource::Version(_) => None,
    }
}

#[allow(dead_code)]
pub fn apply_manifest_to_locked_skill(
    locked_skill: &mut LockedSkill,
    manifest: &crate::manifest::SkillManifest,
) {
    locked_skill.resolved_reference = format!(
        "{} | manifest:{} | version:{} | maturity:{} | verified:{}",
        locked_skill.resolved_reference,
        manifest.manifest_version,
        manifest.skill.version,
        manifest.skill.maturity,
        manifest.skill.last_verified
    );
}

pub fn skill_summary(
    name: &str,
    skill: &ConfigSkill,
    lockfile: Option<&Lockfile>,
    tmp_root: &Path,
    store: &ContentStore,
) -> SkillSummary {
    let statuses = skill_statuses(name, skill, lockfile, tmp_root, store);
    let locked = lockfile.and_then(|lock: &Lockfile| lock.get_skill(name));
    let manifest = load_manifest_for_config_skill(name, skill, tmp_root);

    SkillSummary {
        name: name.to_string(),
        asset_type: "skill".to_string(),
        source: describe_skill_source(skill),
        enabled: skill.enabled,
        statuses: format_statuses(&statuses)
            .split(',')
            .map(ToString::to_string)
            .collect(),
        outdated: super::state::format_outdated_state(classify_outdated(skill, locked)).to_string(),
        lock_summary: describe_locked_skill(locked),
        manifest_version: manifest
            .as_ref()
            .map(|entry| entry.manifest_version.clone()),
        skill_version: manifest.as_ref().map(|entry| entry.skill.version.clone()),
        maturity: manifest.as_ref().map(|entry| entry.skill.maturity.clone()),
        last_verified: manifest
            .as_ref()
            .map(|entry| entry.skill.last_verified.clone()),
        description: manifest
            .as_ref()
            .map(|entry| entry.skill.description.clone()),
    }
}

pub fn asset_summary(
    name: &str,
    asset_type: &str,
    skill: &ConfigSkill,
    lockfile: Option<&Lockfile>,
    tmp_root: &Path,
    store: &ContentStore,
) -> SkillSummary {
    let mut summary = skill_summary(name, skill, lockfile, tmp_root, store);
    summary.asset_type = asset_type.to_string();
    summary
}
