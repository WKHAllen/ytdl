//! A tool to download content from YouTube.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod classes;
mod components;
mod constants;
mod services;

use crate::components::App;
use crate::constants::*;
use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;
use dioxus::desktop::tao::window::Icon;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

fn main() {
    let image_icon =
        image::load_from_memory_with_format(WINDOW_ICON, image::ImageFormat::Ico).unwrap();
    let icon = Icon::from_rgba(
        image_icon.to_rgba8().to_vec(),
        image_icon.width(),
        image_icon.height(),
    )
    .unwrap();

    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_menu(None)
                .with_disable_context_menu(!DEBUG)
                .with_icon(icon.clone())
                .with_window(
                    WindowBuilder::new()
                        .with_always_on_bottom(false)
                        .with_always_on_top(false)
                        .with_title(WINDOW_TITLE)
                        .with_window_icon(Some(icon.clone()))
                        .with_taskbar_icon(Some(icon)),
                ),
        )
        .launch(App);
}