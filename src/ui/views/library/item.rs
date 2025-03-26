use gpui::*;

use crate::events::UiEvent;
use crate::services::AssistantConfig;
use crate::state::*;
use crate::ui::*;

pub struct Item {
    assistant: AssistantConfig,
}

impl Item {
    pub fn new(_cx: &mut Context<Self>, assistant: AssistantConfig) -> Self {
        Item { assistant }
    }
}

impl Render for Item {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                let assistant_id = this.assistant.id.clone();
                cx.emit(UiEvent::SelectAssistant(assistant_id));
            }
        });

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .text_size(theme.text_size)
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.assistant.name.clone()),
            )
            .child(
                div()
                    .text_size(theme.subtext_size)
                    .text_color(theme.muted_foreground)
                    .child(self.assistant.description.clone()),
            )
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, on_click)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for Item {}
