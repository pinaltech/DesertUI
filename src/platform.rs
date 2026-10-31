use gpui::{Window, WindowAppearance};
pub fn system_dark(w: &Window) -> bool {
    matches!(
        w.appearance(),
        WindowAppearance::Dark | WindowAppearance::VibrantDark
    )
}
/// Shortcut text for this platform: macOS symbols, or Ctrl/Alt/Shift words.
pub fn shortcut_text(text: &str) -> String {
    if cfg!(target_os = "macos") {
        return text.into();
    }
    text.replace("⌃ ⌘", "Ctrl Alt")
        .replace("⌃⌘", "Ctrl+Alt+")
        .replace("⌘ ", "Ctrl ")
        .replace('⌘', "Ctrl+")
        .replace("⌥ ", "Alt ")
        .replace('⌥', "Alt+")
        .replace("⇧ ", "Shift ")
        .replace('⇧', "Shift+")
        .replace('⌫', "Backspace")
}
