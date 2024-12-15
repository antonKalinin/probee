use crate::state::{ActiveView, State};
use crate::theme::Theme;
use gpui::*;

pub struct Intro {
    visible: bool,
}

const INTRO_TEXT: &str = "\
Cmdi is a shortcut to ask AI for help.

• Cmd + I to run selected command
• Cmd + Shift + I to hide the app
";

impl Intro {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == ActiveView::AppView;
            cx.notify();
        })
        .detach();

        Intro { visible: true }
    }
}

impl Render for Intro {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        div()
            .line_height(theme.line_height)
            .w_full()
            .mt_4()
            .px_1()
            .text_color(theme.text)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(INTRO_TEXT.to_string())
            .into_any_element()
    }
}
