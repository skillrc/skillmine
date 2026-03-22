use crate::config::ConfigSkill;
use crate::resolved_state::LockedSkill;

pub fn short_hash(value: &str) -> String {
    value.chars().take(8).collect()
}

pub fn describe_skill_source(skill: &ConfigSkill) -> String {
    match &skill.source {
        crate::config::SkillSource::GitHub {
            repo,
            path,
            branch,
            tag,
            commit,
        } => {
            let mut parts = vec![format!("GitHub: {}", repo)];
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
        crate::config::SkillSource::Local { path } => format!("Local path: {}", path),
        crate::config::SkillSource::Version(version) => format!("Version: {}", version),
    }
}

pub fn describe_locked_skill(locked: Option<&LockedSkill>) -> String {
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
