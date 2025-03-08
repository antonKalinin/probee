use gpui::*;
use std::time::Duration;

use crate::services::{Auth, Storage};
use crate::state::*;
use crate::theme::Theme;
use crate::ui::TextInput;

use super::utils;

pub struct LoginView {
    visible: bool,
    enabled: bool,
    email: Option<String>,
    email_input: Entity<TextInput>,
}

const EMAIL_STORAGE_KEY: &str = "recent_email";

impl LoginView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let data = model.read(cx);
            this.visible = data.active_view == ActiveView::LoginView && !data.authenticated;

            if !data.authenticated {
                this.email = None;
            }

            cx.notify();
        })
        .detach();

        let storage = cx.global::<Storage>();
        let recent_email = storage.get(EMAIL_STORAGE_KEY.into());

        let email_input =
            cx.new(|cx| TextInput::new(recent_email, Some("Enter your email".into()), cx));

        cx.spawn(|this, mut cx| async move {
            loop {
                this.update(&mut cx, |this, cx| {
                    let input_value = this.email_input.read(cx).get_content();
                    this.enabled = utils::is_valid_email(&input_value);
                })
                .ok();

                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
            }
        })
        .detach();

        LoginView {
            enabled: false,
            visible: false,
            email: None,

            email_input,
        }
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let theme = cx.global::<Theme>();

        let handle_login = cx.listener(move |this, _event, _window, cx: &mut Context<Self>| {
            if !this.enabled {
                return;
            }

            let input_value = this.email_input.read(cx).get_content();
            let email = match utils::is_valid_email(input_value.trim()) {
                true => input_value.trim().to_string(),
                false => {
                    // TODO: change style of input to error
                    return;
                }
            };

            this.email = Some(email.clone());
            this.email_input.update(cx, |input, _cx| input.reset());
            cx.notify();

            let auth = cx.global::<Auth>().clone();
            let storage = cx.global::<Storage>().clone();

            // Save recently used email to storage to prefill the input on next login
            let _ = storage.set(EMAIL_STORAGE_KEY.into(), email.clone());

            cx.spawn(|_this, mut cx| async move {
                let login_result = auth.login_with_email(&mut cx, email.as_str()).await;

                match login_result {
                    Ok(user) => {
                        set_user_async(&mut cx, Some(user));
                        set_authenticated_async(&mut cx, true);
                        set_active_view_async(&mut cx, ActiveView::ProfileView);
                    }
                    Err(err) => {
                        set_error_async(&mut cx, Some(err));
                    }
                };
            })
            .detach();
        });

        let handle_login_retry =
            cx.listener(move |this, _event, _window, cx: &mut Context<Self>| {
                this.email = None;
                //this.email_input.update(cx, |input, _cx| input.reset());
                cx.notify();
            });

        let title = div()
            .mb_2()
            .text_size(theme.heading_size)
            .text_color(theme.foreground)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::SEMIBOLD)
            .child("Login");

        let instructions = div()
            .mb_4()
            .text_size(theme.subtext_size)
            .text_color(theme.muted_foreground)
            .child("To use public or personal assistants you need to login.")
            .child("If you don't have an account we will create one for you.");

        let send_button = div()
            .w_auto()
            .mt_2()
            .px_4()
            .py_2()
            .rounded_lg()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .bg(self
                .enabled
                .then(|| theme.primary)
                .unwrap_or(theme.muted_foreground))
            .text_color(theme.primary_foreground)
            .hover(|style| {
                style.bg(self
                    .enabled
                    .then(|| theme.primary.opacity(0.9))
                    .unwrap_or(theme.muted_foreground))
            })
            .cursor(match self.enabled {
                true => CursorStyle::PointingHand,
                false => CursorStyle::OperationNotAllowed,
            })
            .on_mouse_up(MouseButton::Left, handle_login)
            .cursor(CursorStyle::PointingHand)
            .child("Login with Magic Link");

        let try_again_button = div()
            .w_auto()
            .mt_2()
            .px_4()
            .py_2()
            .rounded_lg()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .bg(theme.secondary)
            .text_color(theme.secondary_foreground)
            .hover(|style| style.bg(theme.secondary.opacity(0.9)))
            .cursor(CursorStyle::PointingHand)
            .on_mouse_up(MouseButton::Left, handle_login_retry)
            .child("Try with another email");

        let email_sent_notice = div()
            .mt_4()
            .mb_2()
            .flex()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .justify_center()
            .text_size(theme.subtext_size)
            .text_color(theme.primary)
            .font_weight(FontWeight::SEMIBOLD)
            .child(div().child(format!(
                "Magic link sent to {}",
                self.email.clone().unwrap_or("".into())
            )));

        div()
            .line_height(theme.line_height)
            .w_full()
            .my_2()
            .px_1()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::NORMAL)
            .child(title)
            .child(instructions)
            .child(
                self.email
                    .is_some()
                    .then(|| email_sent_notice)
                    .unwrap_or_else(|| div().child(self.email_input.clone())),
            )
            .child(
                self.email
                    .is_some()
                    .then(|| try_again_button)
                    .unwrap_or_else(|| send_button),
            )
            .into_any_element()
    }
}
