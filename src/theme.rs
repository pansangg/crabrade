use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::{App, Hsla, px, rgb};

/// The messenger paints every screen on this background.
pub const BACKGROUND: u32 = 0x121212;

/// Installs the crabrade look: a dark palette on the shared background.
pub fn init(cx: &mut App) {
    Theme::change(ThemeMode::Dark, None, cx);
    Theme::update(cx, |theme| {
        theme.colors.background = Hsla::from(rgb(BACKGROUND));
        // Client-side decorated windows (GNOME Wayland) get rounded corners.
        // Server-decorated windows ignore this and keep the shell's own frame.
        theme.window_radius = px(12.);
    });
}
