//! Global application constants.

use const_format::concatcp;

/// Whether this is a debug build.
pub const DEBUG: bool = cfg!(debug_assertions);

/// The application window title.
pub const WINDOW_TITLE: &str = "YTDL";

/// The application window icon.
pub const WINDOW_ICON: &[u8] = include_bytes!("../assets/img/icon.ico");

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
