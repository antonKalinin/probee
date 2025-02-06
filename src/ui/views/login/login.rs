use gpui::*;

use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::TextInput;

pub struct LoginView {
    visible: bool,
    email_input: Entity<TextInput>,
}

impl LoginView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let data = model.read(cx);
            this.visible = data.active_view == ActiveView::LoginView && !data.authenticated;
            cx.notify();
        })
        .detach();

        let email_input = cx.new(|cx| TextInput::new(String::from("Enter your email"), cx));

        LoginView {
            visible: false,
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

        let click_handle = cx.listener({
            move |_this, _event, _window, _cx: &mut Context<Self>| {
                println!("LOGIN");
            }
        });

        let title = div()
            .mb_2()
            .text_size(theme.heading_size)
            .text_color(theme.foreground)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::SEMIBOLD)
            .child("Login into Command I");

        let instructions = div()
            .mb_4()
            .text_size(theme.subtext_size)
            .text_color(theme.muted_foreground)
            .child("To use public or personal assistants you need to login. If you don't have an account we will create one for you. To manage your assistants please use cmdi.com");

        let button = div()
            .w_auto()
            .mt_2()
            .px_4()
            .py_2()
            .rounded_lg()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .bg(theme.primary)
            .hover(|style| style.bg(theme.accent_foreground))
            .text_color(theme.primary_foreground)
            .cursor(CursorStyle::PointingHand)
            .on_mouse_up(MouseButton::Left, click_handle)
            .cursor(CursorStyle::PointingHand)
            .child("Login with Magic Link");

        div()
            .line_height(theme.line_height)
            .w_full()
            .my_4()
            .px_1()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::NORMAL)
            .child(title)
            .child(instructions)
            .child(self.email_input.clone())
            .child(button)
            .into_any_element()
    }
}
