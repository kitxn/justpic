//! Responsible for working with custom app configuration

use std::path::{Path, PathBuf};

use crate::error::Result;

const DEFAULT_HOST_ADDR: &str = "0.0.0.0:8080";

const DEFAULT_DATA_DIR: &str = "./data";

const DEFAULT_MEDIA_DIR: &str = "./media";

const DEFAULT_DATABASE_PATH: &str = "./data.db";

/// Application Configuration
///
/// Contains all application settings
#[derive(Debug, Clone)]
pub struct Configuration {
    host_addr: String,

    data_dir: PathBuf,
    media_dir: PathBuf,
    db_path: PathBuf,
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

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn media_dir(&self) -> &Path {
        &self.media_dir
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let data_dir = PathBuf::from(DEFAULT_DATA_DIR);

        Configuration {
            host_addr: DEFAULT_HOST_ADDR.to_string(),
            data_dir: data_dir.clone(),
            media_dir: data_dir.join(DEFAULT_MEDIA_DIR),
            db_path: PathBuf::from(DEFAULT_DATABASE_PATH),
        }
    }
}
