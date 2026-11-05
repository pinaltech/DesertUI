//! Composable GPUI application chrome, with Picacho's design as the default.
//!
//! Applications own entities, actions, storage and focus lifetimes. Components
//! return ordinary GPUI elements, so listeners and content stay type checked.
pub mod components;
pub mod image_viewport;
pub mod layout;
pub mod library;
pub mod metrics;
pub mod native_titlebar;
pub mod native_transfer;
pub mod platform;
pub mod theme;
pub use components::*;
pub use layout::{LayoutPolicy, PaneLayout};
pub use platform::{shortcut_text, system_dark};
pub use theme::{Accent, Appearance, Colors};

/// Initialize GPUI Component once during application startup (before views).
pub fn init(cx: &mut gpui::App) {
    gpui_component::init(cx);
}
