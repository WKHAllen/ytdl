//! Application state configuration.

use crate::constants::*;
use crate::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Returns the path to the configuration file.
fn config_file_path() -> Result<PathBuf> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));
    let joined = Path::new(here).join(CONFIG_FILE_NAME);
    Ok(joined)
}

/// The application state configuration with all fields optional.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct ConfigOpt {
    /// The video URL.
    video_url: Option<String>,
    /// The selected content type.
    content_type: Option<ContentType>,
    /// The selected output directory.
    output_directory: Option<PathBuf>,
}

impl From<Config> for ConfigOpt {
    fn from(value: Config) -> Self {
        Self {
            video_url: Some(value.video_url),
            content_type: Some(value.content_type),
            output_directory: value.output_directory,
        }
    }
}

/// The application state configuration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Config {
    /// The video URL.
    pub video_url: String,
    /// The selected content type.
    pub content_type: ContentType,
    /// The selected output directory.
    pub output_directory: Option<PathBuf>,
}

impl From<ConfigOpt> for Config {
    fn from(value: ConfigOpt) -> Self {
        Self {
            video_url: value
                .video_url
                .unwrap_or_else(|| "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned()),
            content_type: value.content_type.unwrap_or(ContentType::Video),
            output_directory: value.output_directory.or_else(|| {
                home::home_dir().map(|path| {
                    let downloads_path = path.join("Downloads");

                    if downloads_path.exists() {
                        downloads_path
                    } else {
                        path
                    }
                })
            }),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from(ConfigOpt::default())
    }
}

impl Config {
    /// Loads the configuration state from the file.
    pub async fn load() -> Result<Self> {
        let config_path = config_file_path()?;

        if config_path.exists() {
            let config_bytes = fs::read(config_path).await?;
            let config_opt = serde_json::from_slice::<ConfigOpt>(&config_bytes)?;
            Ok(Self::from(config_opt))
        } else {
            Ok(Self::default())
        }
    }

    /// Saves the configuration state to the file.
    pub async fn save(&self) -> Result<()> {
        let config_path = config_file_path()?;
        let config_opt = ConfigOpt::from(self.clone());
        let config_bytes = serde_json::to_vec(&config_opt)?;
        fs::write(config_path, config_bytes).await?;
        Ok(())
    }
}
