mod api;
mod session;
mod theme;
mod views;

use gpui_kit::*;

use views::login::LoginView;

fn main() {
    application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            init(cx);
            theme::init(cx);

            let bounds = Bounds::centered(None, size(px(920.), px(640.)), cx);

            open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(480.), px(420.))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("crabrade".into()),
                        ..Default::default()
                    }),
                    // Prefer the shell's own title bar wherever it can draw one
                    // (X11, KDE Wayland, macOS, Windows). GNOME Wayland does not
                    // implement xdg-decoration, so LoginView draws its own bar
                    // when the platform falls back to client-side decorations.
                    window_decorations: Some(WindowDecorations::Server),
                    app_id: Some("crabrade".into()),
                    ..Default::default()
                },
                cx,
                |window, cx| cx.new(|cx| LoginView::new(window, cx)),
            )
            .expect("failed to open the authorization window");
        });
}
