//! Download orchestration component.

use dioxus::prelude::*;

/// Download configuration and trigger component.
pub fn Downloader() -> Element {
    rsx! {
        div {
            class: "downloader-container",

            div {
                class: "downloader",

                "downloader configuration",
            }
        }
    }
}
