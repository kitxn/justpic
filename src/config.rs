//! Responsible for working with custom app configuration

use std::path::{Path, PathBuf};

const DEFAULT_HOST_ADDR: &str = "0.0.0.0:8080";

const DEFAULT_DATA_DIR: &str = "./data";

const DEFAULT_MEDIA_DIR: &str = "./media";

const DEFAULT_TEMP_DIR: &str = "./temp";

const DEFAULT_DATABASE_PATH: &str = "./data.db";

/// Application Configuration
///
/// Contains all application settings
#[derive(Clone)]
pub struct Configuration {
    // TODO: Restructure the application configuration and finally start applying it
    host_addr: String,

    data_dir: PathBuf,
    media_dir: PathBuf,
    temp_dir: PathBuf,

    db_path: PathBuf,
}

impl Configuration {
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

    pub fn temp_dir(&self) -> &PathBuf {
        &self.temp_dir
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let data_dir = PathBuf::from(DEFAULT_DATA_DIR);

        Configuration {
            host_addr: DEFAULT_HOST_ADDR.to_string(),
            data_dir: data_dir.clone(),
            media_dir: data_dir.join(DEFAULT_MEDIA_DIR),
            temp_dir: data_dir.join(DEFAULT_TEMP_DIR),
            db_path: PathBuf::from(DEFAULT_DATABASE_PATH),
        }
    }
}
