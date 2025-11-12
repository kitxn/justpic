//! Responsible for working with custom app configuration

use crate::error::Result;

const DEFAULT_HOST_ADDR: &str = "0.0.0.0:8080";

const DEFAULT_DATA_DIR: &str = "./data";

/// Application Configuration
///
/// Contains all application settings
#[derive(Debug, Clone)]
pub struct Configuration {
    host_addr: String,
}

impl Configuration {
    /// Load application [`Configuration`] from standard config path (`./config.json`)
    pub fn load_from_default_file() -> Result<Self> {
        todo!()
    }

    /// Save the current application [`Configuration`] to the standard config path (`./config.json`)
    pub fn save_to_default() -> Result<()> {
        todo!()
    }

    pub fn host_addr(&self) -> &str {
        &self.host_addr
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            host_addr: DEFAULT_HOST_ADDR.to_string(),
        }
    }
}
