use crate::state::State;
use crate::theme::Theme;
use gpui::*;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            if Some(error) = model.read(cx).error {
                this.message = error.to_string();
                cx.notify();
            }
        })
        .detach();

        Error {
            message: "".to_string(),
        }
    }
}

impl Render for Error {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let text = self.message.clone();

        div().child(text)
    }
}
