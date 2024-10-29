//! UI component for selecting an output directory.

use crate::components::ControlError;
use crate::hooks::*;
use dioxus::prelude::*;
use std::path::PathBuf;

/// A selector for the downloader output directory.
#[component]
pub fn OutputDirectorySelector(
    /// The output directory state.
    state: Signal<Option<PathBuf>>,
) -> Element {
    let id = use_id();
    let display_text = match state() {
        Some(path) => path.display().to_string(),
        None => "No output directory selected".to_owned(),
    };
    let error = state()
        .is_none()
        .then(|| "No output directory selected".to_owned());

    rsx! {
        div {
            class: "output-directory-selector-container",

            span {
                class: "output-directory-selector-label",
                "Output directory"
            }

            div {
                class: "output-directory-selector",

                div {
                    class: "output-directory-selector-display",
                    "{display_text}"
                }

                label {
                    class: "output-directory-selector-button primary",
                    r#for: "{id}",
                    "Browse"
                }

                input {
                    id: "{id}",
                    class: "output-directory-selector-input",
                    r#type: "file",
                    directory: true,
                    onchange: move |event| {
                        if let Some(file_engine) = event.files() {
                            if let Some(path) = file_engine.files().first() {
                                state.set(Some(PathBuf::from(path)));
                            }
                        }
                    }
                }
            }

            ControlError {
                message: error
            }
        }
    }
}
