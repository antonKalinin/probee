use gpui::*;

use crate::events::UiEvent;
use crate::state::State;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct CopyOutputButton {
    enabled: bool,
}

impl CopyOutputButton {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        let _ = cx
            .observe(state, move |this, state: Model<State>, cx| {
                this.enabled = !state.read(cx).output.is_empty();
                cx.notify();
            })
            .detach();

        CopyOutputButton { enabled: false }
    }
}

impl Render for CopyOutputButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let icon_color = match self.enabled {
            true => theme.text,
            false => theme.subtext,
        };

        let icon = svg()
            .path(Icon::Copy.path())
            .text_color(icon_color)
            .hover(|style| style.text_color(theme.text))
            .size_full();

        let on_click = cx.listener({
            move |_this, _event, cx: &mut ViewContext<Self>| {
                cx.emit(UiEvent::CopyOutput);
            }
        });

        let button = div()
            .h_6()
            .w_6()
            .p_1()
            .rounded_full()
            .border_1()
            .on_mouse_up(MouseButton::Left, on_click)
            .hover(|style| style.bg(theme.secondary_hover))
            .cursor(CursorStyle::PointingHand)
            .child(icon);

        button
    }
}

impl EventEmitter<UiEvent> for CopyOutputButton {}
