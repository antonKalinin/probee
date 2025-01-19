use crate::state::{ActiveView, State};
use crate::theme::Theme;
use gpui::*;

pub struct Login {
    visible: bool,
}

const TEXT: &str = "\
Please check yor email for the sign in link. \
";

impl Login {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == ActiveView::LoginView;
            cx.notify();
        })
        .detach();

        Login { visible: true }
    }
}

impl Render for Login {
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
            .child(TEXT.to_owned())
            .into_any_element()
    }
}
