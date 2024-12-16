use crate::state::{ActiveView, State};
use crate::theme::Theme;
use gpui::*;

pub struct Output {
    visible: bool,
    text: String,
}

const HINT_TEXT: &str = "Please, copy some text and press CMD + I";

impl Output {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let loading = model.read(cx).loading;

            this.text = model.read(cx).output.clone();
            this.visible = model.read(cx).active_view == ActiveView::AssitantView && !loading;
            cx.notify();
        })
        .detach();

        Output {
            visible: false,
            text: HINT_TEXT.to_string(),
        }
    }
}

impl Render for Output {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let text = self.text.clone();

        if !self.visible {
            return div().into_any_element();
        }

        if text.is_empty() {
            return div()
                .flex()
                .flex_col()
                .mt_2()
                .w_full()
                .h_16()
                .items_center()
                .justify_center()
                .text_color(theme.subtext)
                .text_size(theme.text_size)
                .child(HINT_TEXT.to_string())
                .into_any_element();
        }

        div()
            .line_height(theme.line_height)
            .w_full()
            .mt_2()
            .px_1()
            .text_color(theme.text)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(text)
            .into_any_element()
    }
}
