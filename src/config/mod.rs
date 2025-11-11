pub mod manager;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub default_shell: Option<String>,
    pub auto_sync: bool,
    pub backup_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_shell: None,
            auto_sync: true,
            backup_enabled: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub settings: Settings,
}
