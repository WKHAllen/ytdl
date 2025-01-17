//! API interfacing with the youtube-dl binary.

use crate::constants::*;
use crate::types::*;
use anyhow::Result;
use image::ImageReader;
use std::env::current_exe;
use std::fmt::Display;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use tokio::process::Command;

/// An error occurring during a download operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DownloadError {
    /// A description of the error.
    description: String,
    /// The output of the download operation. This is typically the `stderr` log
    /// of the youtube-dl binary invocation.
    output: String,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.description, self.output)
    }
}

impl std::error::Error for DownloadError {}

/// Returns the title of the requested video using the youtube-dl binary.
async fn video_title(video_id: &str) -> Result<String> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));

    let mut cmd = Command::new(YOUTUBE_DL_BINARY_NAME);
    cmd.arg("--get-title").arg(video_id);

    #[cfg(windows)]
    {
        cmd.creation_flags(CREATE_NO_WINDOW_FLAG);
    }

    let res = cmd.current_dir(here).output().await?;

    if res.status.success() {
        Ok(String::from_utf8(res.stdout)?.trim().to_owned())
    } else {
        Err(DownloadError {
            description: "failed to fetch video title".to_owned(),
            output: String::from_utf8_lossy(&res.stderr).into_owned(),
        }
        .into())
    }
}

/// Returns the title of the requested video, but with special characters
/// replaced with underscores.
async fn filename_video_title(video_id: &str) -> Result<String> {
    video_title(video_id).await.map(|title| {
        title
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    })
}

/// Downloads the requested video using the youtube-dl binary.
async fn download_thumbnail(video_id: &str, output_directory: &Path) -> Result<PathBuf> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));
    let video_name = filename_video_title(video_id).await?;
    let output_path = output_directory.join(format!("{}.png", video_name));

    let mut cmd = Command::new(YOUTUBE_DL_BINARY_NAME);
    cmd.arg("--get-thumbnail").arg(video_id);

    #[cfg(windows)]
    {
        cmd.creation_flags(CREATE_NO_WINDOW_FLAG);
    }

    let res = cmd.current_dir(here).output().await?;

    if res.status.success() {
        let video_thumbnail_url = String::from_utf8(res.stdout)?.trim().to_owned();
        let bytes = reqwest::get(video_thumbnail_url).await?.bytes().await?;
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;
        img.save_with_format(&output_path, image::ImageFormat::Png)?;
        Ok(output_path)
    } else {
        Err(DownloadError {
            description: "failed to fetch video thumbnail URL".to_owned(),
            output: String::from_utf8_lossy(&res.stderr).into_owned(),
        }
        .into())
    }
}

/// Downloads the requested video using the youtube-dl binary.
async fn download_audio(video_id: &str, output_directory: &Path) -> Result<PathBuf> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));
    let temp_video_dir = TempDir::new()?;
    let video_path = download_video(video_id, temp_video_dir.path()).await?;
    let video_name = video_path
        .with_extension("mp3")
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .into_owned();
    let output_path = output_directory.join(video_name);

    let mut cmd = Command::new(FFMPEG_BINARY_NAME);
    cmd.arg("-i").arg(&video_path).arg(&output_path);

    #[cfg(windows)]
    {
        cmd.creation_flags(CREATE_NO_WINDOW_FLAG);
    }

    let res = cmd.current_dir(here).output().await?;

    if res.status.success() {
        Ok(output_path)
    } else {
        Err(DownloadError {
            description: "failed to convert video to audio file".to_owned(),
            output: String::from_utf8_lossy(&res.stderr).into_owned(),
        }
        .into())
    }
}

/// Downloads the requested video using the youtube-dl binary.
async fn download_video(video_id: &str, output_directory: &Path) -> Result<PathBuf> {
    let current = current_exe()?;
    let here = current.parent().unwrap_or(Path::new("."));
    let video_name = filename_video_title(video_id).await?;
    let output_path = output_directory.join(format!("{}.mp4", video_name));

    let mut cmd = Command::new(YOUTUBE_DL_BINARY_NAME);
    cmd.arg("--format")
        .arg("mp4")
        .arg("--output")
        .arg(&output_path)
        .arg(video_id);

    #[cfg(windows)]
    {
        cmd.creation_flags(CREATE_NO_WINDOW_FLAG);
    }

    let res = cmd.current_dir(here).output().await?;

    if res.status.success() {
        Ok(output_path)
    } else {
        Err(DownloadError {
            description: "failed to perform video download".to_owned(),
            output: String::from_utf8_lossy(&res.stderr).into_owned(),
        }
        .into())
    }
}

/// Downloads the requested content using the youtube-dl binary.
pub async fn download(
    video_id: &str,
    content_type: ContentType,
    output_directory: impl AsRef<Path>,
) -> Result<PathBuf> {
    let output_directory = output_directory.as_ref();

    match content_type {
        ContentType::Thumbnail => download_thumbnail(video_id, output_directory).await,
        ContentType::Audio => download_audio(video_id, output_directory).await,
        ContentType::Video => download_video(video_id, output_directory).await,
    }
}
