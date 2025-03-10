use gpui::*;

use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::Icon;

pub struct Footer {
    visible: bool,
}

impl Footer {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == ActiveView::AssitantView
                && model.read(cx).output.is_empty();

            cx.notify();
        })
        .detach();

        Footer { visible: false }
    }
}

fn cmd_icon(theme: Theme) -> Div {
    div().h_3().w_3().child(
        svg()
            .path(Icon::Command.path())
            .text_color(theme.muted_foreground)
            .size_full(),
    )
}

impl Render for Footer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let theme = cx.global::<Theme>();

        let cmd_i_key = div()
            .flex()
            .items_center()
            .h_5()
            .px_1()
            .bg(theme.secondary)
            .rounded_md()
            .border_1()
            .border_color(theme.border)
            .children([
                cmd_icon(theme.clone()),
                div().child(" + "),
                cmd_icon(theme.clone()),
            ]);

        let cmd_i_shortcut = div()
            .flex()
            .flex_row()
            .children([cmd_i_key, div().ml_1().child("Run Assistant")]);

        div()
            .flex()
            .flex_row()
            .w_full()
            .mt_4()
            .text_color(theme.muted_foreground)
            .text_size(theme.subtext_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .children([cmd_i_shortcut])
            .into_any_element()
    }
}
