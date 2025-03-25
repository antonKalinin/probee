use gpui::*;

use crate::events::UiEvent;
use crate::services::AssistantConfig;
use crate::state::*;
use crate::ui::Theme;

pub struct AssistantHeader {
    assistant: Option<AssistantConfig>,
}

impl AssistantHeader {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let _ = cx
            .observe(state, |this, model, cx| {
                if let Some(assistant_id) = model.read(cx).active_assistant_id.clone() {
                    this.assistant = model
                        .read(cx)
                        .assistants
                        .iter()
                        .find(|a| a.id == assistant_id)
                        .cloned();
                    cx.notify();
                }
            })
            .detach();

        AssistantHeader { assistant: None }
    }
}

impl Render for AssistantHeader {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if self.assistant.is_none() {
            // TODO: push to select assistant
            return div().into_any_element();
        }

        let assistant = self.assistant.as_ref().unwrap();

        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .px_1()
            .text_size(theme.text_size)
            .font_weight(FontWeight::MEDIUM)
            .child(assistant.name.clone())
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for AssistantHeader {}
