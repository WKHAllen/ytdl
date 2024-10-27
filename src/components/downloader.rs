//! Download orchestration component.

use crate::components::{ContentType, ContentTypeSelector, TextInput};
use dioxus::prelude::*;

/// Download configuration and trigger component.
#[component]
pub fn Downloader() -> Element {
    let video_url = use_signal(String::new);
    let content_type = use_signal(|| ContentType::Video);

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
                    required: true,
                }

                ContentTypeSelector {
                    state: content_type,
                }
            }
        }
    }
}
