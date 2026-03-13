use crate::config::{Config, ConfigSkill, SkillSource};
use std::path::PathBuf;

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
    tokio::fs::create_dir_all(config_dir).await?;

    let default_config = r#"version = "1.0"

[skills]
# Add your skills here
# Example: git-commit = { repo = "anthropic/skills", path = "git-release" }
"#;

    tokio::fs::write(&config_path, default_config).await?;
    
    println!("✓ Created configuration at {:?}", config_path);

    Ok(())
}

pub async fn add(
    repo: String,
    branch: Option<String>,
    tag: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::registry::GitClient;

    let config_path = Config::find_config()?;
    let mut config = Config::load(&config_path)?;

    let (repo_name, path) = GitClient::parse_github_ref(&repo)
        .ok_or("Invalid repository format. Use 'owner/repo' or 'owner/repo/path'")?;

    let skill_name = path
        .clone()
        .unwrap_or_else(|| repo_name.split('/').next_back().unwrap_or(&repo).to_string());

    let skill = ConfigSkill {
        source: SkillSource::GitHub {
            repo: repo_name,
            path,
            branch,
            tag,
            commit: None,
        },
        name: Some(skill_name.clone()),
    };

    config.add_skill(&skill_name, skill);
    config.save(&config_path)?;

    println!("✓ Added '{}' to skills.toml", skill_name);
    println!("  Run 'skillmine install' to install it");

    Ok(())
}

pub async fn install(_verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing skills... (not yet fully implemented)");
    Ok(())
}

pub async fn sync(target: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Syncing to {}... (not yet fully implemented)", target);
    Ok(())
}
