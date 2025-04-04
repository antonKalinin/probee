use gpui::*;
use std::time::Duration;

use crate::events::UiEvent;
use crate::state::app::*;
use crate::ui::{Icon, Theme};

pub struct ClearOutputButton {
    enabled: bool,
    succeeded: bool,
}

impl ClearOutputButton {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
        let _ = cx
            .observe(state, move |this, state, cx| {
                this.enabled = !state.read(cx).output.is_empty();
                cx.notify();
            })
            .detach();

        ClearOutputButton {
            enabled: false,
            succeeded: false,
        }
    }
}

impl Render for ClearOutputButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let icon_color = match self.enabled {
            true => theme.foreground,
            false => theme.muted_foreground,
        };

        let icon = svg()
            .path(if self.succeeded {
                Icon::Check.path()
            } else {
                Icon::CircleX.path()
            })
            .text_color(icon_color)
            .hover(|style| style.text_color(theme.foreground))
            .size_full();

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                this.succeeded = true;
                cx.notify();
                cx.emit(UiEvent::ClearOutput);

                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    let _ = this.update(cx, |this, cx| {
                        this.succeeded = false;
                        cx.notify();
                    });
                })
                .detach();
            }
        });

        let button = div()
            .h_6()
            .w_6()
            .p_1()
            .rounded_full()
            .border_1()
            .on_mouse_down(MouseButton::Left, on_click)
            .hover(|style| style.bg(theme.muted))
            .cursor_pointer()
            .child(icon);

        button
    }
}

impl EventEmitter<UiEvent> for ClearOutputButton {}
