use gpui::*;

use crate::events::UiEvent;
use crate::state::ActiveView;
use crate::state::State;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct ProfileButton {
    active: bool,
    authenticated: bool,
}

impl ProfileButton {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        let authenticated = state.read(cx).authenticated;
        let active = state.read(cx).active_view == ActiveView::ProfileView;

        let _ = cx
            .observe(state, move |this, state: Model<State>, cx| {
                this.authenticated = state.read(cx).authenticated;
                this.active = state.read(cx).active_view == ActiveView::ProfileView;
                cx.notify();
            })
            .detach();

        ProfileButton {
            authenticated,
            active,
        }
    }
}

impl Render for ProfileButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.authenticated {
            let click_handle = cx.listener({
                move |_this, _event, cx: &mut ViewContext<Self>| {
                    cx.emit(UiEvent::ChangeActiveView(ActiveView::LoginView));
                }
            });

            return div()
                .w_auto()
                .text_size(theme.subtext_size)
                .text_color(theme.text)
                .hover(|style| style.text_color(theme.subtext))
                .on_mouse_up(MouseButton::Left, click_handle)
                .cursor(CursorStyle::PointingHand)
                .child("Sign in");
        }

        let click_handle = cx.listener({
            move |_this, _event, cx: &mut ViewContext<Self>| {
                cx.emit(UiEvent::ChangeActiveView(ActiveView::ProfileView));
            }
        });

        let icon_color = match self.active {
            true => theme.text,
            false => theme.subtext,
        };

        let button = div()
            .flex()
            .on_mouse_up(MouseButton::Left, click_handle)
            .cursor(CursorStyle::PointingHand)
            .child(
                svg()
                    .path(Icon::CircleUserRound.path())
                    .hover(|style| style.text_color(theme.text))
                    .text_color(icon_color)
                    .size_4(),
            );

        button
    }
}

impl EventEmitter<UiEvent> for ProfileButton {}
