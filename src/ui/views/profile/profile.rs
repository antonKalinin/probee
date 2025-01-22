use crate::state::{ActiveView, State};
use crate::theme::Theme;
use gpui::*;

pub struct ProfileView {
    visible: bool,
}

const TEXT: &str = "\
You are authenticated now. \
";

impl ProfileView {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let data = model.read(cx);
            this.visible = data.active_view == ActiveView::ProfileView && data.authenticated;
            cx.notify();
        })
        .detach();

        ProfileView { visible: false }
    }
}

impl Render for ProfileView {
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
