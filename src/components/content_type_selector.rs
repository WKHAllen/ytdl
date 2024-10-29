//! UI component to enable selection between video, audio, and thumbnail
//! downloading.

use crate::classes::*;
use crate::types::*;
use dioxus::prelude::*;

/// A component to enable selection between video, audio, and thumbnail
/// downloading.
#[component]
pub fn ContentTypeSelector(
    /// The content type state.
    state: Signal<ContentType>,
) -> Element {
    let thumbnail_option_class = classes!(
        "content-type-selector-option",
        matches!(state(), ContentType::Thumbnail)
            .then_some("content-type-selector-option-selected")
    );
    let audio_option_class = classes!(
        "content-type-selector-option",
        matches!(state(), ContentType::Audio).then_some("content-type-selector-option-selected")
    );
    let video_option_class = classes!(
        "content-type-selector-option",
        matches!(state(), ContentType::Video).then_some("content-type-selector-option-selected")
    );

    rsx! {
        div {
            class: "content-type-selector-container",

            span {
                class: "content-type-selector-label",
                "Content type to download"
            }

            div {
                class: "content-type-selector",

                div {
                    class: "{thumbnail_option_class}",
                    onclick: move |_| state.set(ContentType::Thumbnail),
                    "Thumbnail"
                }

                div {
                    class: "{audio_option_class}",
                    onclick: move |_| state.set(ContentType::Audio),
                    "Audio"
                }

                div {
                    class: "{video_option_class}",
                    onclick: move |_| state.set(ContentType::Video),
                    "Video"
                }
            }
        }
    }
}
