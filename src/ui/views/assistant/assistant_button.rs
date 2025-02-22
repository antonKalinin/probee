use gpui::*;

use crate::api::AssistantConfig;
use crate::events::UiEvent;
use crate::state::State;
use crate::theme::Theme;
use crate::ui::Icon;

pub struct AssistantButton {
    active: bool,
    assistant: AssistantConfig,
}

impl AssistantButton {
    pub fn new(cx: &mut Context<Self>, config: AssistantConfig, state: &Entity<State>) -> Self {
        let assistant_id = config.id.clone();
        let active = state.read(cx).active_assistant_id == Some(assistant_id.clone());

        let _ = cx
            .observe(state, move |this, state, cx| {
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

    fn render_icon(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let icon = Icon::TextCursorInput;

        let text_color = match self.active {
            true => theme.primary_foreground,
            false => theme.secondary_foreground,
        };

        let svg = div()
            .flex()
            .child(svg().path(icon.path()).text_color(text_color).size_4());

        svg.into_any_element()
    }

    fn render_label(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let text_color = match self.active {
            true => theme.primary_foreground,
            false => theme.secondary_foreground,
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                let assistant_id = this.assistant.id.clone();
                cx.emit(UiEvent::ChangeAssistant(assistant_id));
            }
        });

        let bg_color = match self.active {
            true => theme.primary,
            false => theme.background,
        };

        let border_color = match self.active {
            true => theme.primary,
            false => theme.border,
        };

        let bg_hover_color = match self.active {
            true => theme.primary.opacity(0.9),
            false => theme.secondary,
        };

        let button = div()
            .h_6()
            .w_auto()
            .px_2()
            .py_1()
            .border_1()
            .rounded_lg()
            .flex()
            .flex_row()
            .items_center()
            .bg(bg_color)
            .border_color(border_color)
            .hover(|style| style.bg(bg_hover_color))
            .on_mouse_up(MouseButton::Left, on_click)
            .cursor(CursorStyle::PointingHand)
            .child(self.render_icon(cx))
            .child(self.render_label(cx));

        button
    }
}

impl EventEmitter<UiEvent> for AssistantButton {}
