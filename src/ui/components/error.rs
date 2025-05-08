use gpui::*;

use crate::state::ErrorState;
use crate::ui::*;

pub struct ErrorView {
    visible: bool,
    message: String,
}

impl ErrorView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<impl ErrorState + 'static>) -> Self {
        cx.observe(state, |this, state, cx| {
            let error_occured = state.read(cx).get_error().is_some();

            if this.visible != error_occured {
                this.visible = error_occured;
                cx.notify();
            }

            if error_occured {
                this.message = state.read(cx).get_error().unwrap().to_string();
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let red100 = Hsla::from(rgb(0xfee2e2));
        let red500 = Hsla::from(rgb(0xef4444));

        if !self.visible {
            return div().into_any_element();
        }

        let bg_color = linear_gradient(
            180.,
            linear_color_stop(theme.background.opacity(0.), 0.),
            linear_color_stop(red100, 1.),
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
            .child(Icon::new(IconName::HeartCrack).text_color(red500));

        let title = div()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .child(error_title);

        let body = div()
            .w_72()
            .text_color(theme.muted_foreground)
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
