use gpui::*;

use crate::events::UiEvent;
use crate::services::AssistantConfig;
use crate::state::app::*;
use crate::ui::*;

pub struct Header {
    assistant: Option<AssistantConfig>,
}

impl Header {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
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

        Header { assistant: None }
    }
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if self.assistant.is_none() {
            // TODO: push to select assistant
            return div().into_any_element();
        }

        let assistant = self.assistant.as_ref().unwrap();

        let on_click = cx.listener({
            move |_this, _event, _window, cx: &mut Context<Self>| {
                cx.emit(UiEvent::ToggleAssistantLibrary);
            }
        });

        let row = || div().flex().flex_row().flex_wrap().items_center();

        let dropdown_icon = div().h_4().w_4().ml_3().child(
            svg()
                .path(Icon::ChevronDown.path())
                .text_color(theme.foreground)
                .size_full(),
        );

        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .px_1()
            .text_size(theme.text_size)
            .font_weight(FontWeight::MEDIUM)
            .child(row().children(vec![div().child(assistant.name.clone()), dropdown_icon]))
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, on_click)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for Header {}
