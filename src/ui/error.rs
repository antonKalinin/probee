use gpui::*;

use crate::state::State;
use crate::theme::Theme;

pub struct ErrorView {
    visible: bool,
    message: String,
}

impl ErrorView {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let error_occured = model.read(cx).error.is_some();

            if this.visible != error_occured {
                this.visible = error_occured;
                cx.notify();
            }

            if error_occured {
                this.message = model.read(cx).error.as_ref().unwrap().to_string();
                cx.notify();
            }
        })
        .detach();

        ErrorView {
            visible: false,
            message: "".to_string(),
        }
    }
}

impl Render for ErrorView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let title = div()
            .mb_1()
            .text_color(theme.red500)
            .text_size(theme.text_size)
            .child("🙈 Error occurred");

        let body = div()
            .text_color(theme.subtext)
            .text_size(theme.subtext_size)
            .child(self.message.clone());

        return div()
            .flex()
            .flex_col()
            .p_4()
            .w_full()
            .justify_center()
            .child(title)
            .child(body)
            .into_any_element();
    }
}
