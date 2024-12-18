use gpui::*;

use crate::state::State;
use crate::theme::Theme;

pub struct ErrorView {
    message: String,
}

impl ErrorView {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            if let Some(error) = model.read(cx).error.as_ref() {
                this.message = error.to_string();
                cx.notify();
            }
        })
        .detach();

        ErrorView {
            message: "".to_string(),
        }
    }
}

impl Render for ErrorView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let _theme = cx.global::<Theme>();

        if self.message.is_empty() {
            return div().into_any_element();
        }

        div().child(self.message.clone()).into_any_element()
    }
}
