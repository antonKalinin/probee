use crate::state::{ActiveView, State};
use crate::theme::Theme;
use gpui::*;

pub struct LoginView {
    visible: bool,
}

const TEXT: &str = "\
Please check yor email for the sign in link. \
";

impl LoginView {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let data = model.read(cx);
            this.visible = data.active_view == ActiveView::LoginView && !data.authenticated;
            cx.notify();
        })
        .detach();

        LoginView { visible: false }
    }
}

impl Render for LoginView {
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
