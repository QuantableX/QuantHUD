use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to get config directory")]
    NoConfigDir,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

fn get_config_path() -> Result<PathBuf, ConfigError> {
    let config_dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let app_dir = config_dir.join("quanthub");
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    
    Ok(app_dir.join("config.json"))
}

pub fn load_config() -> Result<String, ConfigError> {
    let path = get_config_path()?;
    
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok("{}".to_string())
    }
}

pub fn save_config(config: &str) -> Result<(), ConfigError> {
    let path = get_config_path()?;
    fs::write(path, config)?;
    Ok(())
}

