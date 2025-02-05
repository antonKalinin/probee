use gpui::*;

use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::TextInput;

pub struct LoginView {
    visible: bool,
    email_input: Entity<TextInput>,
}

const TEXT: &str = "\
Please check yor email for the sign in link. \
";

impl LoginView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let data = model.read(cx);
            this.visible = data.active_view == ActiveView::LoginView && !data.authenticated;
            cx.notify();
        })
        .detach();

        let email_input = cx.new(|cx| TextInput::new(String::from("Email"), cx));

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

        div()
            .line_height(theme.line_height)
            .w_full()
            .mt_4()
            .px_1()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(TEXT.to_owned())
            .child(self.email_input.clone())
            .into_any_element()
    }
}
