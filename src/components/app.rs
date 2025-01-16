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
    Completed(Config),
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
            } else {
                update_youtube_dl_binary().await?;
            }

            Config::load().await
        }
        .await;

        match res {
            Ok(config) => dep_fetch_status.set(DepFetchStatus::Completed(config)),
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
                        text: "Installing/updating application dependencies..."
                    }
                },
                DepFetchStatus::Completed(config) => rsx! {
                    Downloader {
                        config: config,
                    }
                },
                DepFetchStatus::Failed(err) => rsx! {
                    Error {
                        class: "dep-fetch-status-failed",
                        description: "An error occurred while installing/updating application dependencies:",
                        message: err
                    }
                }
            }
        }
    }
}
