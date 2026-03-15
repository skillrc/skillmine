use std::path::PathBuf;

use super::settings::Config;

pub fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string_pretty(config)?;
    std::fs::write(path, content)?;
    Ok(())
}

pub fn find_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let local_path = PathBuf::from("skills.toml");
    if local_path.exists() {
        return Ok(local_path);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let global_path = config_dir.join("skillmine").join("skills.toml");
        if global_path.exists() {
            return Ok(global_path);
        }
    }

    Err("No configuration file found. Run 'skillmine init' first.".into())
}
