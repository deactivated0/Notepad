#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(
    clippy::collapsible_if,
    clippy::if_same_then_else,
    clippy::extra_unused_lifetimes,
    clippy::manual_is_ascii_check
)]

mod app;
mod document;
mod icons;
mod io;
mod settings;
mod theme;
mod ui;

use iced::{Size, window};

use crate::app::NotepadApp;

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let window_settings = window::Settings {
        size: Size::new(1024.0, 680.0),
        min_size: Some(Size::new(480.0, 320.0)),
        // Frameless custom Windows 11 tabbed titlebar on desktop (Wayland, X11, Windows, macOS).
        decorations: false,
        resizable: true,
        ..window::Settings::default()
    };

    iced::application(NotepadApp::boot, NotepadApp::update, NotepadApp::view)
        .title(NotepadApp::title)
        .theme(NotepadApp::theme)
        .subscription(NotepadApp::subscription)
        .window(window_settings)
        .antialiasing(true)
        .run()
}
