use gpui::*;

use crate::events::UiEvent;
use crate::state::ActiveView;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct BackButton {
    visible: bool,
}

impl BackButton {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let visible = state.read(cx).active_view != ActiveView::AssitantView;

        let _ = cx
            .observe(state, move |this, state, cx| {
                this.visible = state.read(cx).active_view != ActiveView::AssitantView;
                cx.notify();
            })
            .detach();

        BackButton { visible }
    }
}

impl Render for BackButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div();
        }

        let theme = cx.global::<Theme>();
        let click_handle = cx.listener({
            move |_this, _event, _window, cx: &mut Context<Self>| {
                set_active_view(cx, ActiveView::AssitantView.clone());
            }
        });

        div()
            .flex()
            .text_size(theme.subtext_size)
            .text_color(theme.foreground)
            .on_mouse_up(MouseButton::Left, click_handle)
            .cursor(CursorStyle::PointingHand)
            .child(
                svg()
                    .path(Icon::ChevronLeft.path())
                    .hover(|style| style.text_color(theme.foreground))
                    .text_color(theme.foreground)
                    .size_4(),
            )
            .child("Back")
    }
}

impl EventEmitter<UiEvent> for BackButton {}
