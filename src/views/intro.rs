use crate::state::State;
use crate::theme::Theme;
use gpui::*;

pub struct Intro {}

const INTRO_TEXT: &str = "Hello, this your assistant.\n\n\
- Cmd + I to run selected command
- Cmd + Shift + I to switch assistant mode
- Cmd + Opt + I to hide the assistant";

impl Intro {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.text = model.read(cx).output.clone();
            cx.notify();
        })
        .detach();

        Output {
            text: INTRO_TEXT.to_string(),
        }
    }
}

impl Render for Intro {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let text = self.text.clone();

        if self.text.is_empty() {
            return div().into_any_element();
        }

        div()
            .line_height(theme.line_height)
            .w_full()
            .mt_2()
            .text_color(theme.text)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(text)
            .into_any_element()
    }
}
