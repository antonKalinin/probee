use gpui::*;

use crate::assistant::AssistMode;
use crate::events::UiEvent;
use crate::state::{State, StateController};
use crate::theme::Theme;
use crate::ui::Icon;

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
            AssistMode::PlainFinnish => Icon::Milk,
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
            AssistMode::PlainFinnish => "In Plain Finnish",
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
