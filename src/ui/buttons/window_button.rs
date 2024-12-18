use gpui::*;

use crate::events::UiEvent;
use crate::theme::Theme;

pub enum WindowAction {
    Hide,
    Close,
}

pub struct WindowButton {
    action: WindowAction, // TODO: Maybe use UiEvent subset directly
}

impl WindowButton {
    pub fn new(action: WindowAction) -> Self {
        WindowButton { action }
    }
}

impl Render for WindowButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let bg_color = match self.action {
            WindowAction::Hide => theme.amber400,
            WindowAction::Close => theme.red500,
        };

        let on_click = cx.listener({
            move |this, _event, cx: &mut ViewContext<Self>| {
                let app_event = match this.action {
                    WindowAction::Hide => UiEvent::HideWindow,
                    WindowAction::Close => UiEvent::CloseWindow,
                };

                cx.emit(app_event);
            }
        });

        let button = div()
            .h_3()
            .w_3()
            .rounded_full()
            .bg(bg_color)
            .on_mouse_up(MouseButton::Left, on_click)
            .cursor(CursorStyle::PointingHand);

        button
    }
}

impl EventEmitter<UiEvent> for WindowButton {}
