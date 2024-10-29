//! Service for fetching the dependency binaries from the web.

use crate::constants::*;
use anyhow::Result;
use futures_util::StreamExt;
use std::env::current_exe;
use std::path::{Path, PathBuf};
use tempfile::{tempfile, TempDir};
use tokio::fs::{self, File};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::task::spawn_blocking;
use zip::ZipArchive;

/// Returns the path to the ffmpeg binary.
fn ffmpeg_binary_path() -> Result<PathBuf> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));
    let joined = Path::new(here).join(FFMPEG_BINARY_NAME);
    Ok(joined)
}

/// Checks if the ffmpeg binary exists and is in the current working
/// directory.
pub fn ffmpeg_binary_exists() -> Result<bool> {
    ffmpeg_binary_path().map(|path| path.exists())
}

/// Fetches the ffmpeg binary from GitHub and places it in the current working
/// directory.
pub async fn fetch_ffmpeg_binary() -> Result<()> {
    let mut byte_stream = reqwest::get(FFMPEG_BINARY_URL).await?.bytes_stream();
    let mut temp_zip_file = File::from_std(tempfile()?);

    while let Some(chunk) = byte_stream.next().await {
        let chunk = chunk?;
        temp_zip_file.write_all(&chunk).await?;
    }

    temp_zip_file.rewind().await?;
    let mut archive = ZipArchive::new(temp_zip_file.into_std().await)?;
    let temp_unzip_dir = TempDir::new()?;
    let temp_unzip_path = temp_unzip_dir.path().to_path_buf();
    spawn_blocking(move || archive.extract(temp_unzip_path)).await??;

    let unzipped_ffmpeg_binary_path = temp_unzip_dir
        .path()
        .join("ffmpeg-master-latest-win64-gpl")
        .join("bin")
        .join(FFMPEG_BINARY_NAME);
    fs::copy(unzipped_ffmpeg_binary_path, ffmpeg_binary_path()?).await?;

    Ok(())
}

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