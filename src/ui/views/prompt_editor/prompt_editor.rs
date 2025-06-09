use gpui::*;

use crate::assistant::Prompt;
use crate::services::{Storage, StorageKey};
use crate::ui::{Button, InputEvent, Sizable as _, TextInput, Theme};

pub struct PromptEditorView {
    prompt: Option<Prompt>,
    name_input: Entity<TextInput>,
    prompt_input: Entity<TextInput>,
    // on_close: Option<Box<dyn Fn(&String, &mut Window, &mut App) + 'static>>,
    save_enabled: bool,
}

impl PromptEditorView {
    pub fn new(
        prompt: Option<Prompt>,
        on_close: impl Fn(&bool, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_name = prompt.as_ref().map(|p| p.name.clone()).unwrap_or("".into());
        let prompt_text = prompt
            .as_ref()
            .map(|p| p.system_message.clone())
            .unwrap_or("".into());

        let name_input = cx.new(|cx| {
            let mut text_input = TextInput::new(window, cx).placeholder("What I should do?");
            text_input.set_text(prompt_name, window, cx);
            text_input
        });

        let prompt_input = cx.new(|cx| {
            let mut text_input = TextInput::new(window, cx)
                .placeholder("You are an expert in ... ")
                .multi_line()
                .rows(20);

            text_input.set_text(prompt_text, window, cx);
            text_input
        });

        cx.subscribe(
            &name_input,
            |this, input, event, cx| {
                if let InputEvent::Change(text) = event {}
            },
        )
        .detach();

        window.on_window_should_close(cx, move |window, cx| {
            on_close(&true, window, cx);
            true
        });

        PromptEditorView {
            prompt,
            name_input,
            prompt_input,
            // on_close: Some(Box::new(on_close)),
            save_enabled: false,
        }
    }

    pub fn build(
        prompt: Option<Prompt>,
        on_close: impl Fn(&bool, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| PromptEditorView::new(prompt, on_close, window, cx))
    }
}

impl Render for PromptEditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let row = || div().w_full().flex().flex_row().mb_6().items_center();

        let label = |text: &str| {
            div()
                .w(px(128.))
                .text_align(TextAlign::Right)
                .text_size(theme.subtext_size)
                .text_color(theme.muted_foreground)
                .font_weight(FontWeight::MEDIUM)
                .mr_6()
                .child(text.to_owned())
        };

        let value = || div().w(px(360.));

        let save_prompt_button = div().child(
            Button::new("create-prompt-button")
                .label("Save Prompt")
                .small()
                .flex()
                .w_32()
                .on_click(
                    cx.listener(|this, _event, _window, cx: &mut Context<Self>| {
                        let name = this.name_input.read(cx).text().clone();
                        let text = this.prompt_input.read(cx).text().clone();

                        if name.is_empty() || text.is_empty() {
                            return;
                        }

                        let prompt = match &this.prompt {
                            Some(existing_prompt) => existing_prompt
                                .to_owned()
                                .set_name(name.into())
                                .set_message(text.into())
                                .clone(),
                            None => Prompt::new(name.into(), text.into()),
                        };

                        let storage = cx.global_mut::<Storage>();
                        let existing_prompts: Vec<Prompt> = storage
                            .get(StorageKey::Prompts)
                            .and_then(|value| serde_json::from_str(&value).ok())
                            .unwrap_or(vec![]);

                        // Replace prompt if it exists by id or append if it doesn't
                        let mut updated_prompts = existing_prompts
                            .into_iter()
                            .filter(|p| p.id != prompt.id)
                            .collect::<Vec<_>>();

                        updated_prompts.push(prompt.clone());

                        let _ = storage.set(
                            StorageKey::Prompts,
                            serde_json::to_string(&updated_prompts).unwrap(),
                        );
                    }),
                ),
        );

        div()
            .w_full()
            .h_full()
            .py_16()
            .bg(theme.background)
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(row().children(vec![label("Name"), value().child(self.name_input.clone())]))
            .child(
                row()
                    .items_start()
                    .children(vec![
                        label("Prompt").pt_1(),
                        value().child(self.prompt_input.clone()),
                    ])
                    .mt_8(),
            )
            .child(
                row()
                    .items_start()
                    .children(vec![
                        label(""),
                        value()
                            .flex()
                            .flex_row()
                            .justify_center()
                            .child(save_prompt_button),
                    ])
                    .mt_8(),
            )
            .into_any_element()
    }
}
