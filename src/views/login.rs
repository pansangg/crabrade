use gpui_kit::component::{
    ActiveTheme as _, Sizable as _, TitleBar,
    button::{Button, ButtonVariants as _},
    input::{Input, InputState},
    label::Label,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use crate::theme::BACKGROUND;

/// The form column is one width, and every control in it fills that width.
const FORM_WIDTH: Pixels = px(360.);
const FORM_GAP: Pixels = px(12.);
/// A large input is 44px tall, while a large button is 32px, so the button
/// gets this height explicitly to keep the two controls the same size.
const CONTROL_HEIGHT: Pixels = px(44.);

pub struct LoginView {
    login: Entity<InputState>,
    password: Entity<InputState>,
    /// Shown under the button when the login request fails. `None` keeps the
    /// label invisible while its row still reserves the space.
    error: Option<SharedString>,
}

impl LoginView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let login = cx.new(|cx| InputState::new(window, cx).placeholder("Логин"));
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Пароль")
                .masked(true)
        });

        Self {
            login,
            password,
            error: None,
        }
    }

    /// Surfaces a failed login under the button. Kept for the API wiring.
    #[allow(dead_code)]
    pub fn set_error(&mut self, error: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.error = Some(error.into());
        cx.notify();
    }
}

impl Render for LoginView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let error_color = cx.theme().danger;
        let border_color = cx.theme().border;
        let has_error = self.error.is_some();
        // GNOME Wayland does not implement xdg-decoration, so the platform falls
        // back to client-side decorations and the shell draws no title bar. Draw
        // one ourselves only in that case: X11, KWin and other compositors that
        // grant server-side decorations keep their own native title bar.
        let client_decorated = cfg!(target_os = "linux")
            && matches!(window.window_decorations(), Decorations::Client { .. });

        let form = div()
            .flex()
            .flex_col()
            .w(FORM_WIDTH)
            .gap(FORM_GAP)
            .child(
                Label::new("crabrade")
                    .w_full()
                    .text_center()
                    .text_size(px(30.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xf5f5f5))
                    .mb(px(16.)),
            )
            .child(Input::new(&self.login).large().w_full())
            .child(Input::new(&self.password).large().w_full())
            .child(
                Button::new("login")
                    .primary()
                    .large()
                    .label("Войти")
                    .w_full()
                    .h(CONTROL_HEIGHT),
            )
            .child(
                div()
                    .w_full()
                    .h(px(20.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .opacity(if has_error { 1. } else { 0. })
                    .child(
                        Label::new(self.error.clone().unwrap_or_default())
                            .text_sm()
                            .text_color(error_color),
                    ),
            );

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BACKGROUND))
            .when(client_decorated, |this| {
                this.child(
                    TitleBar::new()
                        .bg(rgb(BACKGROUND))
                        .border_color(border_color)
                        .child(
                            Label::new("crabrade")
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xf5f5f5)),
                        ),
                )
            })
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(form),
            )
    }
}
