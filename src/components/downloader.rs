//! Download orchestration component.

use crate::components::{Button, ContentTypeSelector, OutputDirectorySelector, TextInput};
use crate::services::download;
use crate::types::*;
use dioxus::prelude::*;

/// Parses and returns the ID portion of a youtube video URL.
fn parse_video_id(video_id: String) -> Option<String> {
    // TODO: do this right
    video_id.split("v=").last().map(ToOwned::to_owned)
}

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
    let video_url = use_signal(String::new);
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

    let perform_download = move |_| {
        spawn(async move {
            status.set(DownloadStatus::Running);

            let video_id = match parse_video_id(video_url()) {
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
                        disabled: matches!(status(), DownloadStatus::Running),
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
