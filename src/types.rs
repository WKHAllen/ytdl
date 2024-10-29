//! Types used in multiple places across the application.

/// The download content type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContentType {
    /// The video thumbnail.
    Thumbnail,
    /// The audio of the video.
    Audio,
    /// The entire video, with audio.
    Video,
}
