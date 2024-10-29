//! Download orchestration component.

use crate::components::{
    Button, ButtonStyle, ContentTypeSelector, OutputDirectorySelector, TextInput,
};
use crate::services::{download, parse_video_url};
use crate::types::*;
use dioxus::prelude::*;

/// The status of a download operation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
enum DownloadStatus {
    /// No download has occurred or is occurring.
    #[default]
    Init,
    /// A download operation is running.
    Running,
    /// A download operation was successful.
    Success(String),
    /// A download operation failed.
    Failure(String),
}

/// Download configuration and trigger component.
#[component]
pub fn Downloader() -> Element {
    let video_url = use_signal(|| "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_owned());
    let content_type = use_signal(|| ContentType::Video);
    let output_directory = use_signal(|| {
        home::home_dir().map(|path| {
            let downloads_path = path.join("Downloads");

            if downloads_path.exists() {
                downloads_path
            } else {
                path
            }
        })
    });
    let mut status = use_signal(DownloadStatus::default);

    let video_url_value = video_url();
    let video_id = parse_video_url(&video_url_value);
    let video_url_error = if video_url_value.is_empty() {
        Some("No URL provided".to_owned())
    } else {
        video_id.is_none().then(|| "Invalid YouTube URL".to_owned())
    };

    let allow_download = video_id.is_some()
        && output_directory().is_some()
        && !matches!(status(), DownloadStatus::Running);

    let perform_download = move |_| {
        let video_id = video_id.clone();
        spawn(async move {
            status.set(DownloadStatus::Running);

            let video_id = match video_id {
                Some(video_id) => video_id,
                None => {
                    status.set(DownloadStatus::Failure(
                        "unknown video URL format".to_owned(),
                    ));
                    return;
                }
            };

            let output_dir = match output_directory() {
                Some(output_dir) => output_dir,
                None => {
                    status.set(DownloadStatus::Failure(
                        "no output directory specified".to_owned(),
                    ));
                    return;
                }
            };

            let res = download(&video_id, content_type(), &output_dir).await;

            match res {
                Ok(_) => status.set(DownloadStatus::Success(format!(
                    "Saved to {}",
                    output_dir
                        .components()
                        .last()
                        .map(|component| component.as_os_str().to_string_lossy().into_owned())
                        .unwrap_or_else(|| "folder".to_owned())
                ))),
                Err(err) => {
                    status.set(DownloadStatus::Failure(err.to_string()));
                }
            }
        });
    };

    rsx! {
        div {
            class: "downloader-container",

            div {
                class: "downloader",

                h1 {
                    class: "downloader-title",
                    "YouTube Downloader"
                }

                TextInput {
                    state: video_url,
                    label: "Video URL",
                    placeholder: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
                    error: video_url_error,
                }

                ContentTypeSelector {
                    state: content_type,
                }

                OutputDirectorySelector {
                    state: output_directory,
                }

                div {
                    class: "download-button-container",

                    Button {
                        text: "Download",
                        class: "download-button",
                        style: ButtonStyle::Primary,
                        disabled: !allow_download,
                        onclick: perform_download,
                    }
                }

                div {
                    class: "download-status",

                    match status() {
                        DownloadStatus::Init => rsx! {
                            span { }
                        },
                        DownloadStatus::Running => rsx! {
                            span { "Performing download..." }
                        },
                        DownloadStatus::Success(message) => rsx! {
                            span { "{message}" }
                        },
                        DownloadStatus::Failure(message) => rsx! {
                            span { "Download failed: {message}" }
                        },
                    }
                }
            }
        }
    }
}
