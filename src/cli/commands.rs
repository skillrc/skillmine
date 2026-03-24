use std::collections::BTreeMap;
use std::path::Path;

use crate::installer::ContentStore;
use crate::source_refs::GitClient;

use super::diagnostics::{print_diagnostic, skill_health_lines, DiagnosticLevel};
use super::pure::{describe_locked_skill, describe_skill_source};
use super::state::{classify_outdated, format_outdated_state};
use super::summary::{load_manifest_for_config_skill, skill_summary};

fn lifecycle_stage(summary: &crate::cli::SkillSummary) -> String {
    summary.statuses.join("+")
}

#[allow(dead_code)]
pub async fn info(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = super::config_and_lockfile()?;
    let tmp_root = super::tmp_root()?;
    let store = ContentStore::default();
    let skill = config
        .skills
        .get(&name)
        .ok_or_else(|| format!("Skill '{}' not found", name))?;
    let summary = skill_summary(&name, skill, lockfile.as_ref(), &tmp_root, &store);

    println!("Skill: {}", name);
    println!("Source: {}", describe_skill_source(skill));
    println!("Enabled: {}", skill.enabled);
    println!(
        "Outdated: {}",
        format_outdated_state(classify_outdated(
            skill,
            lockfile.as_ref().and_then(|lock| lock.get_skill(&name))
        ))
    );
    println!(
        "Lock: {}",
        describe_locked_skill(lockfile.as_ref().and_then(|lock| lock.get_skill(&name)))
    );
    if let Some(version) = summary.skill_version {
        println!("Skill Version: {}", version);
    }
    if let Some(maturity) = summary.maturity {
        println!("Maturity: {}", maturity);
    }
    if let Some(last_verified) = summary.last_verified {
        println!("Last Verified: {}", last_verified);
    }
    if let Some(description) = summary.description {
        println!("Description: {}", description);
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn outdated() -> Result<(), Box<dyn std::error::Error>> {
    let (_, config, _, lockfile) = super::config_and_lockfile()?;

    if config.skills.is_empty() {
        println!("No skills configured.");
        return Ok(());
    }

    match lockfile {
        Some(lockfile) => {
            for (name, skill) in &config.skills {
                if !skill.enabled {
                    println!("{}: disabled", name);
                    continue;
                }
                let state = classify_outdated(skill, lockfile.get_skill(name));
                println!("{}: {}", name, format_outdated_state(state));
            }
        }
        None => {
            for name in config.skills.keys() {
                let skill = config.skills.get(name).unwrap();
                if !skill.enabled {
                    println!("{}: disabled", name);
                } else {
                    println!("{}: missing-from-lock", name);
                }
            }
        }
    }

    Ok(())
}

pub async fn doctor() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config, lockfile_path, lockfile) = super::config_and_lockfile()?;
    let store_path = ContentStore::default_path()?;
    let tmp_path = super::tmp_root()?;
    let mut pass_count = 0;
    let mut inactive_count = 0;
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

            if ContentStore::default().get(&locked.resolved_tree_hash).is_none() {
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
                && !GitClient::has_resolvable_head(&tmp_repo)
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
        if !skill.enabled {
            print_diagnostic(
                DiagnosticLevel::Pass,
                format!("skill '{}' is intentionally inactive; runtime operations skipped", name),
            );
            inactive_count += 1;
            continue;
        }
        if !skill.sync_enabled {
            print_diagnostic(
                DiagnosticLevel::Pass,
                format!("skill '{}' is runtime inactive; sync operations skipped", name),
            );
            inactive_count += 1;
            continue;
        }

        let manifest = load_manifest_for_config_skill(name, skill, &tmp_path);
        match manifest {
            Some(manifest) => {
                print_diagnostic(
                    DiagnosticLevel::Pass,
                    format!(
                        "skill '{}' manifest v{} metadata present (version {}, maturity {})",
                        name,
                        manifest.manifest_version,
                        manifest.skill.version,
                        manifest.skill.maturity
                    ),
                );
                pass_count += 1;
            }
            None => {
                print_diagnostic(
                    DiagnosticLevel::Warn,
                    format!("skill '{}' missing SKILL.toml manifest; using legacy/fallback mode", name),
                );
                warn_count += 1;
            }
        }

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
            for (level, message) in skill_health_lines(
                name,
                skill,
                Some(lockfile_ref),
                &tmp_path,
                &ContentStore::default(),
            ) {
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
        "Summary: {} pass, {} inactive, {} warn, {} fail",
        pass_count, inactive_count, warn_count, fail_count
    );

    if fail_count > 0 {
        return Err(format!("doctor found {} failing checks", fail_count).into());
    }

    Ok(())
}

pub async fn doctor_summary() -> Result<String, Box<dyn std::error::Error>> {
    let (config_path, config, lockfile_path, lockfile) = super::config_and_lockfile()?;
    let tmp_path = super::tmp_root()?;
    let mut pass_count = 0;
    let mut inactive_count = 0;
    let mut warn_count = 0;
    let mut fail_count = 0;
    let mut lines = vec![
        format!("Configuration: {}", config_path.display()),
        format!("Version: {}", config.version),
        format!("Skills configured: {}", config.skills.len()),
        format!("Lockfile exists: {}", lockfile_path.exists()),
    ];

    if config.validate().is_ok() {
        pass_count += 1;
        lines.push("PASS config validation".to_string());
    } else {
        fail_count += 1;
        lines.push("FAIL config validation".to_string());
    }

    for (name, skill) in &config.skills {
        if !skill.enabled {
            lines.push(format!("{} :: intentionally-inactive :: disabled", name));
            inactive_count += 1;
            continue;
        }
        if !skill.sync_enabled {
            lines.push(format!("{} :: runtime-inactive :: unsynced", name));
            inactive_count += 1;
            continue;
        }

        let summary = skill_summary(
            name,
            skill,
            lockfile.as_ref(),
            &tmp_path,
            &ContentStore::default(),
        );
        if summary.outdated == "up-to-date" {
            pass_count += 1;
        } else {
            warn_count += 1;
        }
        lines.push(format!(
            "{} :: lifecycle: {}",
            name,
            lifecycle_stage(&summary)
        ));
        lines.push(format!("{} :: outdated: {}", name, summary.outdated));
        lines.push(format!("{} :: lock: {}", name, summary.lock_summary));
    }

    lines.push(format!(
        "Summary: {} pass, {} inactive, {} warn, {} fail",
        pass_count, inactive_count, warn_count, fail_count
    ));
    Ok(lines.join("\n"))
}

pub async fn clean(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = super::data_root()?;
    let tmp_dir = data_dir.join("tmp");
    match std::fs::remove_dir_all(&tmp_dir) {
        Ok(()) => {
            println!("✓ Removed temporary directory {}", tmp_dir.display());
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }

    if all {
        let store_dir = data_dir.join("store");
        match std::fs::remove_dir_all(&store_dir) {
            Ok(()) => {
                println!("✓ Removed store directory {}", store_dir.display());
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
