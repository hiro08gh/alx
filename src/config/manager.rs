use crate::config::Config;
use crate::error::{AlxError, Result};
use std::fs;
use std::path::PathBuf;

pub struct ConfigManager {
    config_dir: PathBuf,
    config_file: PathBuf,
    aliases_file: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AlxError::ConfigError("Could not find config directory".to_string()))?
            .join("alx");

        let config_file = config_dir.join("config.toml");
        let aliases_file = config_dir.join("aliases.toml");

        Ok(Self {
            config_dir,
            config_file,
            aliases_file,
        })
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn config_file(&self) -> &PathBuf {
        &self.config_file
    }

    pub fn aliases_file(&self) -> &PathBuf {
        &self.aliases_file
    }

    pub fn shell_dir(&self) -> PathBuf {
        self.config_dir.join("shell")
    }

    pub fn shell_aliases_file(&self) -> PathBuf {
        self.shell_dir().join("aliases.sh")
    }

    pub fn backup_dir(&self) -> PathBuf {
        self.config_dir.join("backups")
    }

    pub fn init(&self) -> Result<()> {
        // Create config directory
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(self.shell_dir())?;
        fs::create_dir_all(self.backup_dir())?;

        // Create default config if not exists
        if !self.config_file.exists() {
            let config = Config::default();
            self.save_config(&config)?;
        }

        // Create empty aliases file if not exists
        if !self.aliases_file.exists() {
            fs::write(&self.aliases_file, "aliases = []\n")?;
        }

        Ok(())
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        let content =
            toml::to_string_pretty(config).map_err(|e| AlxError::ConfigError(e.to_string()))?;
        fs::write(&self.config_file, content)?;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.config_dir.exists() && self.config_file.exists()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_manager() -> (ConfigManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("alx");
        let config_file = config_dir.join("config.toml");
        let aliases_file = config_dir.join("aliases.toml");

        let manager = ConfigManager {
            config_dir,
            config_file,
            aliases_file,
        };

        (manager, temp_dir)
    }

    #[test]
    fn test_init() {
        let (manager, _temp) = create_test_manager();
        assert!(manager.init().is_ok());
        assert!(manager.config_dir().exists());
        assert!(manager.config_file().exists());
        assert!(manager.aliases_file().exists());
    }

    #[test]
    fn test_save_and_load_config() {
        let (manager, _temp) = create_test_manager();
        manager.init().unwrap();

        let config = Config::default();
        assert!(manager.save_config(&config).is_ok());
    }
}
