use crate::config::{Config, ConfigSkill, SkillSource};
use crate::error::SkillmineError;
use git2::Repository;
use semver::{Version, VersionReq};
use std::process::Command;

pub fn resolve_version_source(
    name: &str,
    skill: &ConfigSkill,
    config: &Config,
) -> Result<ConfigSkill, SkillmineError> {
    match &skill.source {
        SkillSource::Version(version) => {
            let entry = config.registry.get(name).ok_or_else(|| {
                SkillmineError::Registry(format!(
                    "Version skill '{}' is missing a registry entry",
                    name
                ))
            })?;

            let selected_tag = resolve_version_tag(&entry.repo, version)?;

            Ok(ConfigSkill {
                source: SkillSource::GitHub {
                    repo: entry.repo.clone(),
                    path: entry.path.clone(),
                    branch: None,
                    tag: Some(selected_tag),
                    commit: None,
                },
                name: skill.name.clone().or_else(|| Some(name.to_string())),
                enabled: true,
                sync_enabled: true,
            })
        }
        _ => Ok(skill.clone()),
    }
}

fn resolve_version_tag(repo: &str, constraint: &str) -> Result<String, SkillmineError> {
    let tags = if let Ok(repository) = Repository::open(repo) {
        let names = repository
            .tag_names(Some("v*"))
            .map_err(|e| SkillmineError::Registry(format!("Failed to list tags: {}", e)))?;
        names.iter().flatten().map(|tag| tag.to_string()).collect()
    } else {
        fetch_remote_tags(repo)?
    };

    select_matching_tag(&tags, constraint).ok_or_else(|| {
        SkillmineError::Registry(format!(
            "No tag in '{}' satisfies version constraint '{}'",
            repo, constraint
        ))
    })
}

fn fetch_remote_tags(repo: &str) -> Result<Vec<String>, SkillmineError> {
    let url = crate::source_refs::pure::github_repo_url(repo);
    let output = Command::new("git")
        .args(["ls-remote", "--tags", &url])
        .output()
        .map_err(|e| SkillmineError::Registry(format!("Failed to run git ls-remote: {}", e)))?;

    if !output.status.success() {
        return Err(SkillmineError::Registry(format!(
            "git ls-remote failed for '{}': {}",
            url,
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    let mut tags = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(reference) = line.split_whitespace().nth(1) {
            if let Some(tag) = reference.strip_prefix("refs/tags/") {
                if !tag.ends_with("^{}") {
                    tags.push(tag.to_string());
                }
            }
        }
    }

    Ok(tags)
}

fn select_matching_tag(tags: &[String], constraint: &str) -> Option<String> {
    let req = normalize_version_req(constraint)?;
    let mut matches: Vec<(Version, String)> = tags
        .iter()
        .filter_map(|tag| {
            let normalized = tag.strip_prefix('v').unwrap_or(tag);
            let version = Version::parse(normalized).ok()?;
            if req.matches(&version) {
                Some((version, tag.clone()))
            } else {
                None
            }
        })
        .collect();

    matches.sort_by(|left, right| left.0.cmp(&right.0));
    matches.pop().map(|(_, tag)| tag)
}

fn normalize_version_req(constraint: &str) -> Option<VersionReq> {
    VersionReq::parse(constraint).ok().or_else(|| {
        if constraint
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
        {
            VersionReq::parse(&format!("={}", constraint)).ok()
        } else {
            None
        }
    })
}
