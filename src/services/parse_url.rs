//! Utilities for parsing and validating URLs.

use url::{Host, Url};

/// Validates that a video ID is in the expected format.
fn valid_video_id(id: &str) -> bool {
    id.len() == 11
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Parses a YouTube video URL, returning the video ID. If the URL is invalid,
/// `None` is returned.
pub fn parse_video_url(video_url: &str) -> Option<String> {
    let video_url = Url::parse(video_url).ok()?;

    match video_url.host() {
        Some(Host::Domain("youtube.com")) | Some(Host::Domain("www.youtube.com")) => {
            if video_url.path() == "/watch" {
                let video_id = video_url.query_pairs().find(|(key, _)| key == "v")?.1;
                valid_video_id(&video_id).then_some(video_id.into_owned())
            } else {
                None
            }
        }
        Some(Host::Domain("youtu.be")) | Some(Host::Domain("www.youtu.be")) => {
            let video_id = video_url
                .path()
                .strip_prefix('/')
                .unwrap_or(video_url.path());
            valid_video_id(video_id).then_some(video_id.to_owned())
        }
        _ => None,
    }
}
