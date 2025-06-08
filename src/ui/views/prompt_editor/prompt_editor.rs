use gpui::*;

use crate::assistant::Prompt;
use crate::storage::{Storage, StorageKey};
use crate::ui::{Button, Disableable, InputEvent, Sizable as _, TextInput, Theme};

pub struct PromptEditorView {
    prompt: Option<Prompt>,
    name_input: Entity<TextInput>,
    prompt_input: Entity<TextInput>,
    save_enabled: bool,
}

impl PromptEditorView {
    pub fn build(prompt: Option<Prompt>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let storage = cx.global::<Storage>();

        let view = cx.new(move |cx| {
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

            PromptEditorView {
                prompt,
                name_input,
                prompt_input,
                save_enabled: false,
            }
        });

        view
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
                .disabled(!self.save_enabled)
                .on_click(
                    cx.listener(|_this, _event, window, _cx: &mut Context<Self>| {
                        window.remove_window();
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
