use gpui::*;

use crate::state::*;
use crate::ui::Theme;

pub struct LibraryView {
    visible: bool,
}

impl LibraryView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let visible = state.read(cx).active_view == ActiveView::LibraryView;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_view == ActiveView::LibraryView;
            cx.notify();
        })
        .detach();

        LibraryView { visible }
    }
}

impl Render for LibraryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        div()
            .line_height(theme.line_height)
            .w_full()
            .p_1()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .child("Library")
            .into_any_element()
    }
}
