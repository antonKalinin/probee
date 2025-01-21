use gpui::*;

use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::Icon;

pub struct Footer {
    visible: bool,
}

impl Footer {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
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
            .text_color(theme.subtext)
            .size_full(),
    )
}

impl Render for Footer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let theme = cx.global::<Theme>();

        let shift_icon = div().h_3().w_3().child(
            svg()
                .path(Icon::ArrowBigUp.path())
                .text_color(theme.subtext)
                .size_full(),
        );

        let i_key = div().ml_1().child("I");

        let cmd_i_key = div()
            .flex()
            .items_center()
            .h_5()
            .px_1()
            .bg(theme.secondary)
            .rounded_md()
            .border_1()
            .border_color(theme.border_secondary)
            .children([cmd_icon(theme.clone()), div().ml_1().child("I")]);

        let cmd_shift_i_key = div()
            .flex()
            .items_center()
            .h_5()
            .px_1()
            .bg(theme.secondary)
            .rounded_md()
            .border_1()
            .border_color(theme.border_secondary)
            .children([cmd_icon(theme.clone()), shift_icon, i_key]);

        let cmd_i_shortcut = div()
            .flex()
            .flex_row()
            .children([cmd_i_key, div().ml_1().child("Run Assistant")]);

        let cmd_shift_i_shortcut = div()
            .flex()
            .flex_row()
            .ml_4()
            .children([cmd_shift_i_key, div().ml_1().child("Hide Assistant")]);

        div()
            .flex()
            .flex_row()
            .w_full()
            .mt_4()
            .text_color(theme.subtext)
            .text_size(theme.subtext_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .children([cmd_i_shortcut, cmd_shift_i_shortcut])
            .into_any_element()
    }
}
