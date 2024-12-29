use gpui::*;

use crate::api::AssistantConfig;
use crate::events::UiEvent;
use crate::state::{State, StateController};
use crate::theme::Theme;
use crate::ui::Icon;

pub struct AssistantButton {
    active: bool,
    assistant: AssistantConfig,
}

impl AssistantButton {
    pub fn new(cx: &mut ViewContext<Self>, config: AssistantConfig, active: bool) -> Self {
        let state = cx.global::<StateController>().model.clone();
        let assistant_id = config.id.clone();

        let _ = cx
            .observe(&state, move |this, state: Model<State>, cx| {
                if let Some(state_assistant_id) = state.read(cx).active_assistant_id.as_ref() {
                    this.active = state_assistant_id == &assistant_id;
                } else {
                    this.active = false;
                }
                cx.notify();
            })
            .detach();

        AssistantButton {
            active,
            assistant: config,
        }
    }

    fn render_icon(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let icon = Icon::Globe;

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
            .child(self.assistant.name.clone());

        label.into_any_element()
    }
}

impl Render for AssistantButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, cx: &mut ViewContext<Self>| {
                let assistant_id = this.assistant.id.clone();
                cx.emit(UiEvent::ChangeAssistant(assistant_id));
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

impl EventEmitter<UiEvent> for AssistantButton {}
