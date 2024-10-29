//! Global application constants.

use const_format::concatcp;

/// Whether this is a debug build.
pub const DEBUG: bool = cfg!(debug_assertions);

/// The application window title.
pub const WINDOW_TITLE: &str = "YTDL";

/// The application window icon.
pub const WINDOW_ICON: &[u8] = include_bytes!("../assets/img/icon.ico");

/// The ffmpeg binary file name.
#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY_NAME: &str = "ffmpeg.exe";
/// The ffmpeg binary file name.
#[cfg(not(target_os = "windows"))]
pub const FFMPEG_BINARY_NAME: &str = "ffmpeg";

/// The ffmpeg binary URL.
#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip";
/// The ffmpeg binary URL.
#[cfg(not(target_os = "windows"))]
pub const FFMPEG_BINARY_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz";

/// The youtube-dl binary file name.
#[cfg(target_os = "windows")]
pub const YOUTUBE_DL_BINARY_NAME: &str = "youtube-dl.exe";
/// The youtube-dl binary file name.
#[cfg(not(target_os = "windows"))]
pub const YOUTUBE_DL_BINARY_NAME: &str = "youtube-dl";

/// The youtube-dl binary URL.
pub const YOUTUBE_DL_BINARY_URL: &str = concatcp!(
    "https://github.com/ytdl-org/ytdl-nightly/releases/download/2024.08.07/",
    YOUTUBE_DL_BINARY_NAME
);
