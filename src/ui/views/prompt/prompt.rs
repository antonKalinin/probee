use gpui::*;

use crate::storage::{Storage, StorageKey};
use crate::ui::{Button, Sizable as _, TextInput, Theme};

pub struct PromptEditorView {
    name_input: Entity<TextInput>,
    prompt_input: Entity<TextInput>,
}

impl PromptEditorView {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let view = cx.new(move |cx| {
            let storage = cx.global::<Storage>();

            PromptEditorView {
                name_input: cx.new(|cx| {
                    let mut text_input = TextInput::new(window, cx).placeholder("Prompt Name");
                    text_input.set_text("", window, cx);
                    text_input
                }),
                prompt_input: cx.new(|cx| {
                    let mut text_input = TextInput::new(window, cx)
                        .placeholder("Enter your prompt here")
                        .multi_line()
                        .rows(20);

                    text_input.set_text("", window, cx);
                    text_input
                }),
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
                .on_click(cx.listener(|_this, _event, _window, _cx: &mut Context<Self>| {})),
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
