use gpui::*;

use crate::events::UiEvent;
use crate::state::State;
use crate::theme::Theme;

pub struct LoginButton;

impl LoginButton {
    pub fn new(_cx: &mut ViewContext<Self>, _state: &Model<State>) -> Self {
        LoginButton {}
    }
}

impl Render for LoginButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let click_handle = cx.listener({
            move |_this, _event, cx: &mut ViewContext<Self>| {
                cx.emit(UiEvent::Login);
            }
        });

        let button = div()
            .w_auto()
            .text_size(theme.subtext_size)
            .text_color(theme.text)
            .hover(|style| style.text_color(theme.subtext))
            .on_mouse_up(MouseButton::Left, click_handle)
            .cursor(CursorStyle::PointingHand)
            .child("Login");

        button
    }
}

impl EventEmitter<UiEvent> for LoginButton {}
