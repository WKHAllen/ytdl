//! Loading spinner component.

use crate::classes::*;
use dioxus::prelude::*;

/// Loading spinner size.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LoadingSpinnerSize {
    /// A small spinner.
    Small,
    /// A medium size spinner.
    #[default]
    Medium,
    /// A large spinner.
    Large,
    /// A spinner that grows to the size of the container.
    Max,
}

impl LoadingSpinnerSize {
    /// Gets the name of the loading spinner size.
    pub const fn size_name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Max => "max",
        }
    }
}

/// Loading spinner component.
#[component]
pub fn Loading(
    /// Loading text.
    text: Option<String>,
    /// The size of the loading spinner.
    #[props(default)]
    size: LoadingSpinnerSize,
    /// An optional class name for the loading element.
    class: Option<String>,
) -> Element {
    let container_class = classes!("loading-spinner-container", class);
    let svg_class = classes!(
        "loading-spinner",
        format!("loading-spinner-{}", size.size_name())
    );

    rsx! {
        div {
            class: "{container_class}",

            div {
                class: "loading-spinner-inner",

                if let Some(loading_text) = text {
                    span {
                        class: "loading-spinner-text",
                        "{loading_text}"
                    }
                }

                svg {
                    class: "{svg_class}",
                    view_box: "0 0 50 50",

                    circle {
                        class: "spinner-path",
                        cx: 25,
                        cy: 25,
                        r: 20,
                        fill: "none",
                        stroke_width: 5
                    }
                }
            }
        }
    }
}
