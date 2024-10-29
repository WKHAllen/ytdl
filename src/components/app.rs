//! Root-level application component.

use crate::components::{Downloader, Error, Loading};
use crate::services::*;
use anyhow::Error;
use dioxus::prelude::*;
use std::rc::Rc;

/// The global stylesheet asset.
const STYLES: &str = include_str!("../../assets/css/main.css");

/// The status of fetching dependencies.
#[derive(Debug, Clone)]
enum DepFetchStatus {
    /// Currently fetching dependencies.
    Pending,
    /// Fetching dependencies completed successfully.
    Completed,
    /// Fetching dependencies failed with a provided error.
    Failed(Rc<Error>),
}

/// The root-level application component.
#[component]
pub fn App() -> Element {
    let mut dep_fetch_status = use_signal(|| DepFetchStatus::Pending);

    use_future(move || async move {
        let res = async move {
            if !ffmpeg_binary_exists()? {
                fetch_ffmpeg_binary().await?;
            }

            if !youtube_dl_binary_exists()? {
                fetch_youtube_dl_binary().await?;
            }

            Ok(())
        }
        .await;

        match res {
            Ok(_) => dep_fetch_status.set(DepFetchStatus::Completed),
            Err(err) => dep_fetch_status.set(DepFetchStatus::Failed(Rc::new(err))),
        }
    });

    rsx! {
        div {
            class: "app",

            style {
                "{STYLES}"
            }

            match dep_fetch_status() {
                DepFetchStatus::Pending => rsx! {
                    Loading {
                        class: "dep-fetch-status-pending",
                        text: "Installing application dependencies..."
                    }
                },
                DepFetchStatus::Completed => rsx! {
                    Downloader { }
                },
                DepFetchStatus::Failed(err) => rsx! {
                    Error {
                        class: "dep-fetch-status-failed",
                        description: "An error occurred while installing application dependencies:",
                        message: err
                    }
                }
            }
        }
    }
}
