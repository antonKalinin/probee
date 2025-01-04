use gpui::*;

use crate::state::State;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct ErrorView {
    visible: bool,
    message: String,
}

impl ErrorView {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let error_occured = model.read(cx).error.is_some();

            if this.visible != error_occured {
                this.visible = error_occured;
                cx.notify();
            }

            if error_occured {
                this.message = model.read(cx).error.as_ref().unwrap().to_string();
                cx.notify();
            }
        })
        .detach();

        ErrorView {
            visible: false,
            message: "".to_owned(),
        }
    }
}

impl Render for ErrorView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let bg_color = linear_gradient(
            180.,
            linear_color_stop(theme.background.opacity(0.), 0.),
            linear_color_stop(theme.red100, 1.),
        );

        let (error_title, error_body) = self
            .message
            .clone()
            .split_once("\n")
            .map(|(title, body)| (title.to_string(), body.to_string()))
            .unwrap_or((
                String::from("Unknown error"),
                String::from("Something went wrong"),
            ));

        let alert_icon = div()
            .flex()
            .items_center()
            .justify_center()
            .p_2()
            .mr_2()
            .rounded_full()
            .shadow_md()
            .bg(theme.background)
            .child(
                svg()
                    .path(Icon::TriangleAlert.path())
                    .text_color(theme.red500)
                    .w_4()
                    .h_4(),
            );

        let title = div()
            .text_color(theme.text)
            .text_size(theme.text_size)
            .child(error_title);

        let body = div()
            .text_color(theme.subtext)
            .text_size(theme.subtext_size)
            .child(error_body);

        let row = div().flex().flex_row().items_start();
        let col = div().flex().flex_col();

        return div()
            .bg(bg_color)
            .flex()
            .flex_col()
            .p_4()
            .w_full()
            .justify_center()
            .child(row.child(alert_icon).child(col.child(title).child(body)))
            .into_any_element();
    }
}
