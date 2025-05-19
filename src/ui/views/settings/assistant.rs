use gpui::*;

use crate::state::settings_state::*;
use crate::storage::{Storage, StorageKey};
use crate::ui::{
    ActiveTheme, Button, Dropdown, DropdownEvent, DropdownItem, Icon, IconName, InputEvent,
    Sizable as _, TextInput, Theme,
};

use super::components::{Prompt, PromptList};

const VIEW_HEIGHT: f32 = 464.0;

#[derive(Clone)]
enum ModelProvider {
    Anthropic,
    OpenAI,
}

#[derive(Clone)]
struct Model {
    name: SharedString,
    provider: ModelProvider,
}

impl Model {
    pub fn new(name: impl Into<SharedString>, provider: ModelProvider) -> Self {
        Self {
            name: name.into(),
            provider,
        }
    }
}

impl DropdownItem for Model {
    type Value = Model;

    fn title(&self) -> SharedString {
        self.name.clone()
    }

    fn display_title(&self, cx: &App) -> Option<gpui::AnyElement> {
        let element = div()
            .gap_1()
            .flex()
            .flex_row()
            .items_center()
            .child(
                Icon::new(match self.provider {
                    ModelProvider::Anthropic => IconName::Anthropic,
                    ModelProvider::OpenAI => IconName::OpenAi,
                })
                .text_color(cx.theme().primary),
            )
            .child(self.name.clone());

        Some(element.into_any_element())
    }

    fn value(&self) -> &Self::Value {
        &self
    }
}

pub struct AssistantSettingsView {
    api_key_input: Entity<TextInput>,
    model_dropdown: Entity<Dropdown<Vec<Model>>>,
    prompt_list: Entity<PromptList>,

    provider: ModelProvider,
}

impl AssistantSettingsView {
    pub fn new(
        _state: &Entity<SettingsState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let storage = cx.global::<Storage>();

        let models = vec![
            Model::new("Claude 3.7", ModelProvider::Anthropic),
            Model::new("Claude 3.5 Sonnet", ModelProvider::Anthropic),
            Model::new("Claude 3.5 Haiku", ModelProvider::Anthropic),
            Model::new("GPT-4.1", ModelProvider::OpenAI),
            Model::new("GPT-4.1 mini", ModelProvider::OpenAI),
            Model::new("GPT-4.1 nano", ModelProvider::OpenAI),
            Model::new("GPT-4o", ModelProvider::OpenAI),
            Model::new("GPT-4o mini", ModelProvider::OpenAI),
        ];

        let prompts = vec![
            Prompt::new("Change Tone to Confident", ""),
            Prompt::new("Change Tone to Professional", ""),
            Prompt::new("Explain This in Simple Terms", ""),
            Prompt::new("Translate to Chinese", ""),
            Prompt::new("Translate to English", ""),
            Prompt::new("Translate to German", ""),
            Prompt::new("Translate to Spanish", ""),
            Prompt::new("Translate to Russian", ""),
        ];

        let provider = ModelProvider::Anthropic;

        let api_key = (match provider {
            ModelProvider::Anthropic => storage.get(StorageKey::AnthropicApiKey),
            ModelProvider::OpenAI => storage.get(StorageKey::AssistantOpenAiApiKey),
        })
        .unwrap_or(String::from(""));

        let api_key_input = cx.new(|cx| {
            let mut text_input = TextInput::new(window, cx).placeholder("Enter Anthropic API Key");

            text_input.set_text(api_key, window, cx);
            text_input
        });

        cx.subscribe(&api_key_input, |this, input, event, cx| {
            if let InputEvent::Blur = event {
                let api_key = input.read(cx).text();
                let storage = cx.global::<Storage>();
                let storage_key = match this.provider {
                    ModelProvider::Anthropic => StorageKey::AnthropicApiKey,
                    ModelProvider::OpenAI => StorageKey::AssistantOpenAiApiKey,
                };

                let _ = storage.set(storage_key, api_key.into());
            }
        })
        .detach();

        let model_dropdown = cx.new(|cx| {
            Dropdown::new("model-dropdown", models, Some(0), window, cx).placeholder("Select Model")
        });

        cx.subscribe(&model_dropdown, |this, _dropdown, event, cx| {
            let DropdownEvent::Confirm(value) = event;

            if let Some(model) = value {
                this.provider = model.provider.clone();

                this.api_key_input.update(cx, |input, cx| {
                    let placeholder = match model.provider {
                        ModelProvider::Anthropic => "Enter Anthropic API Key",
                        ModelProvider::OpenAI => "Enter OpenAI API Key",
                    };

                    input.set_placeholder(placeholder);
                    // input.set_text("".to_string(), window, cx);
                });

                cx.notify();
            }
        })
        .detach();

        let prompt_list = cx.new(|cx| PromptList::new(prompts, window, cx));

        AssistantSettingsView {
            api_key_input,
            model_dropdown,
            prompt_list,

            provider: ModelProvider::Anthropic,
        }
    }
}

impl Render for AssistantSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let row = || div().w_full().flex().flex_row().mb_6().items_center();

        let label = |text: &str| {
            div()
                .w(px(168.))
                .text_align(TextAlign::Right)
                .text_size(theme.subtext_size)
                .text_color(theme.muted_foreground)
                .font_weight(FontWeight::MEDIUM)
                .mr_6()
                .child(text.to_owned())
        };

        let value = || div().w(px(280.));
        let separator = || div().w_full().border_b_1().border_color(theme.border);

        let create_prompt_button = div().child(
            Button::new("create-prompt-button")
                .label("Create New Prompt")
                .small()
                .on_click(cx.listener({ |this, event, window, cx: &mut Context<Self>| {} })),
        );

        div()
            .w_full()
            .h(px(VIEW_HEIGHT))
            .py_8()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(row().children(vec![
                label("Model"),
                value().child(self.model_dropdown.clone()),
            ]))
            .child(
                row()
                    .children(vec![
                        label("API Key"),
                        value().child(self.api_key_input.clone()),
                    ])
                    .mb_8(),
            )
            .child(separator())
            .child(
                row()
                    .items_start()
                    .children(vec![
                        label("Prompts").pt_1(),
                        value().child(self.prompt_list.clone()),
                    ])
                    .mt_8(),
            )
            .child(
                row()
                    .items_start()
                    .children(vec![label("").pt_1(), value().child(create_prompt_button)])
                    .mt_8(),
            )
            .into_any_element()
    }
}
