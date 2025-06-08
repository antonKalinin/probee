use gpui::*;

use crate::assistant::Prompt;
use crate::events::UiEvent;
use crate::state::app_state::*;
use crate::ui::*;

pub struct Header {
    prompt: Option<Prompt>,
}

impl Header {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
        let _ = cx
            .observe(state, |this, model, cx| {
                if let Some(prompt) = model.read(cx).active_prompt_id.clone() {
                    this.prompt = model
                        .read(cx)
                        .prompts
                        .iter()
                        .find(|a| a.id == prompt)
                        .cloned();

                    cx.notify();
                }
            })
            .detach();

        Header { prompt: None }
    }
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if self.prompt.is_none() {
            // TODO: push to select prompt
            return div().into_any_element();
        }

        let prompt = self.prompt.as_ref().unwrap();

        let on_click = cx.listener({
            move |_this, _event, _window, cx: &mut Context<Self>| {
                cx.emit(UiEvent::ToggleAssistantLibrary);
            }
        });

        let row = || div().flex().flex_row().flex_wrap().items_center();

        let dropdown_icon = Icon::new(IconName::ChevronDown)
            .opacity(0.)
            .text_color(theme.foreground)
            .ml_3()
            .group_hover("prompt-name", |style| style.opacity(1.));

        div()
            .group("prompt-name")
            .flex()
            .flex_row()
            .flex_wrap()
            .px_1()
            .text_size(theme.text_size)
            .font_weight(FontWeight::MEDIUM)
            .child(row().children(vec![
                div().text_color(theme.primary).child(prompt.name.clone()),
                div().child(dropdown_icon),
            ]))
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, on_click)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for Header {}
