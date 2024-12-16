use gpui::*;

use crate::assistant::AssistMode;
use crate::events::UiEvent;
use crate::state::{ActiveView, State, StateController};
use crate::theme::Theme;
use crate::views::Icon;

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

pub struct ModeButton {
    active: bool,
    mode: AssistMode,
}

impl ModeButton {
    pub fn new(cx: &mut ViewContext<Self>, mode: AssistMode, active: bool) -> Self {
        let state = cx.global::<StateController>().model.clone();
        let button_mode = mode.clone();

        let _ = cx
            .observe(&state, move |this, state: Model<State>, cx| {
                if let Some(current_mode) = state.read(cx).mode.as_ref() {
                    this.active = current_mode == &button_mode;
                } else {
                    this.active = false;
                }
                cx.notify();
            })
            .detach();

        ModeButton { active, mode }
    }

    fn render_icon(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let icon = match self.mode {
            AssistMode::Translate => Icon::Globe,
            AssistMode::WordMorphology => Icon::WholeWord,
            AssistMode::ELI5 => Icon::Milk,
        };

        let text_color = match self.active {
            true => theme.text_foreground,
            false => theme.text,
        };

        let svg = div()
            .flex()
            .child(svg().path(icon.path()).text_color(text_color).size_4());

        svg.into_any_element()
    }

    fn render_label(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let text = match self.mode {
            AssistMode::Translate => "Translate",
            AssistMode::WordMorphology => "Word Morphology",
            AssistMode::ELI5 => "ELI5",
        };

        let text_color = match self.active {
            true => theme.text_foreground,
            false => theme.text,
        };

        let label = div()
            .flex()
            .ml_1()
            .pt_1()
            .text_xs()
            .text_color(text_color)
            .child(text);

        label.into_any_element()
    }
}

impl Render for ModeButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, cx: &mut ViewContext<Self>| {
                let mode = this.mode.clone();
                cx.emit(UiEvent::ChangeMode(mode));
            }
        });

        let bg_color = match self.active {
            true => theme.primary,
            false => theme.secondary,
        };

        let bg_hover_color = match self.active {
            true => theme.primary_hover,
            false => theme.secondary_hover,
        };

        let button = div()
            .h_6()
            .w_auto()
            .px_2()
            .py_1()
            .border_1()
            .rounded_full()
            .flex()
            .flex_row()
            .items_center()
            .bg(bg_color)
            .hover(|style| style.bg(bg_hover_color))
            .on_mouse_up(MouseButton::Left, on_click)
            .cursor(CursorStyle::PointingHand)
            .child(self.render_icon(cx))
            .child(self.render_label(cx));

        button
    }
}

impl EventEmitter<UiEvent> for ModeButton {}

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
