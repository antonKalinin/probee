use gpui::*;
use std::time::Duration;

use crate::state::State;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct Loading {
    visible: bool,
}

impl Loading {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).loading.clone();
            cx.notify();
        })
        .detach();

        Loading { visible: false }
    }
}

impl Render for Loading {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let svg = div().flex().child(
            svg()
                .path(Icon::Loader.path())
                .text_color(theme.muted_foreground)
                .size_6()
                .with_animation(
                    "rotating-loader",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |icon, delta| {
                        icon.with_transformation(Transformation::rotate(percentage(delta)))
                    },
                ),
        );

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .h_20()
            .w_full()
            .child(svg)
            .into_any_element()
    }
}
