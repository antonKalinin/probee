use gpui::*;
use std::time::Duration;

use crate::events::UiEvent;
use crate::state::app_state::*;
use crate::ui::{ActiveTheme, Icon, IconName};

pub struct CopyOutputButton {
    enabled: bool,
    succeeded: bool,
}

impl CopyOutputButton {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
        let _ = cx
            .observe(state, move |this, state, cx| {
                this.enabled = !state.read(cx).output.is_empty();
                cx.notify();
            })
            .detach();

        CopyOutputButton {
            enabled: false,
            succeeded: false,
        }
    }
}

impl Render for CopyOutputButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let icon_color = match self.enabled {
            true => theme.foreground.opacity(0.7),
            false => theme.muted_foreground,
        };

        let icon = (if self.succeeded {
            Icon::new(IconName::Check)
        } else {
            Icon::new(IconName::Copy)
        })
        .text_color(icon_color)
        .hover(|style| style.text_color(theme.foreground));

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                this.succeeded = true;
                cx.notify();
                cx.emit(UiEvent::CopyOutput);

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
            .bg(theme.background)
            .on_mouse_down(MouseButton::Left, on_click)
            .cursor(CursorStyle::PointingHand)
            .child(icon);

        button
    }
}

impl EventEmitter<UiEvent> for CopyOutputButton {}
