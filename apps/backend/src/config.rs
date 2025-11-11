//! Responsible for working with custom app configuration

use crate::error::Result;

const DEFAULT_HOST_ADDR: &str = "0.0.0.0";

const DEFAULT_API_PORT: u16 = 8080;
const DEFAULT_WEB_UI_PORT: u16 = 5175;

const DEFAULT_DATA_DIR: &str = "./data";

/// Application Configuration
///
/// Contains all application settings
pub struct Configuration {
    //
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
}
