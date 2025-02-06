use gpui::*;

use crate::events::UiEvent;
use crate::state::ActiveView;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct ProfileButton {
    visible: bool,
    authenticated: bool,
}

impl ProfileButton {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let authenticated = state.read(cx).authenticated;
        let visible = state.read(cx).active_view == ActiveView::AssitantView;

        let _ = cx
            .observe(state, move |this, state, cx| {
                this.authenticated = state.read(cx).authenticated;
                this.visible = state.read(cx).active_view == ActiveView::AssitantView;
                cx.notify();
            })
            .detach();

        ProfileButton {
            authenticated,
            visible,
        }
    }
}

impl Render for ProfileButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div();
        }

        if !self.authenticated {
            let click_handle = cx.listener({
                move |_this, _event, _window, cx: &mut Context<Self>| {
                    set_active_view(cx, ActiveView::LoginView.clone());
                }
            });

            return div()
                .w_auto()
                .text_size(theme.subtext_size)
                .text_color(theme.foreground)
                .hover(|style| style.text_color(theme.muted_foreground))
                .on_mouse_up(MouseButton::Left, click_handle)
                .cursor(CursorStyle::PointingHand)
                .child("Sign in");
        }

        let click_handle = cx.listener({
            move |_this, _event, _window, cx: &mut Context<Self>| {
                set_active_view(cx, ActiveView::ProfileView.clone());
            }
        });

        div()
            .flex()
            .on_mouse_up(MouseButton::Left, click_handle)
            .cursor(CursorStyle::PointingHand)
            .child(
                svg()
                    .path(Icon::CircleUserRound.path())
                    .hover(|style| style.text_color(theme.foreground))
                    .text_color(theme.foreground)
                    .size_4(),
            )
    }
}

impl EventEmitter<UiEvent> for ProfileButton {}
