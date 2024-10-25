//! Service for fetching the youtube-dl binary from GitHub.

use crate::constants::*;
use anyhow::Result;
use std::env::current_exe;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Returns the path to the youtube-dl binary.
fn youtube_dl_binary_path() -> Result<PathBuf> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));
    let joined = Path::new(here).join(YOUTUBE_DL_BINARY_NAME);
    Ok(joined)
}

/// Checks if the youtube-dl binary exists and is in the current working
/// directory.
pub fn youtube_dl_binary_exists() -> Result<bool> {
    youtube_dl_binary_path().map(|path| path.exists())
}

/// Fetches the youtube-dl binary from GitHub and places it in the current
/// working directory.
pub async fn fetch_youtube_dl_binary() -> Result<()> {
    let bytes = reqwest::get(YOUTUBE_DL_BINARY_URL).await?.bytes().await?;
    fs::write(youtube_dl_binary_path()?, bytes).await?;
    Ok(())
}
