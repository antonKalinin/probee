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
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_view == ActiveView::ProfileView && data.authenticated;
            cx.notify();
        })
        .detach();

        ProfileView { visible: false }
    }
}

impl Render for ProfileView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

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
            .into_any_element()
    }
}
