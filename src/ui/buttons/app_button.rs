use gpui::*;

use crate::events::UiEvent;
use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::Icon;

pub struct AppButton {
    active: bool,
}

impl AppButton {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        let _ = cx
            .observe(state, move |this, state: Model<State>, cx| {
                this.active = state.read(cx).active_view == ActiveView::AppView;
                cx.notify();
            })
            .detach();

        AppButton {
            active: state.read(cx).active_view == ActiveView::AppView,
        }
    }
}

impl Render for AppButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let icon_color = match self.active {
            true => theme.text,
            false => theme.subtext,
        };

        let icon = svg()
            .path(Icon::Command.path())
            .text_color(icon_color)
            .hover(|style| style.text_color(theme.text))
            .size_full();

        let on_click = cx.listener({
            move |_this, _event, cx: &mut ViewContext<Self>| {
                cx.emit(UiEvent::ChangeActiveView(ActiveView::AppView));
            }
        });

        let button = div()
            .h_4()
            .w_4()
            .on_mouse_up(MouseButton::Left, on_click)
            .cursor(CursorStyle::PointingHand)
            .child(icon);

        button
    }
}

impl EventEmitter<UiEvent> for AppButton {}
