//! Download orchestration component.

use crate::components::{
    Button, ContentType, ContentTypeSelector, OutputDirectorySelector, TextInput,
};
use dioxus::prelude::*;

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
                        onclick: move |_| {
                            println!("button clicked");
                        },
                    }
                }
            }
        }
    }
}
