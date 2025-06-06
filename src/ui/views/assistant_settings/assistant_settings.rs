use gpui::*;

use crate::assistant::{Model, ModelProvider};
use crate::state::settings_state::*;
use crate::storage::{Storage, StorageKey};
use crate::ui::{
    ActiveTheme, Button, Dropdown, DropdownEvent, DropdownItem, Icon, IconName, InputEvent,
    Sizable as _, TextInput, Theme,
};

use super::components::{Prompt, PromptList};

const VIEW_HEIGHT: f32 = 464.0;

impl DropdownItem for Model {
    type Value = Model;

    fn title(&self) -> SharedString {
        self.title.clone()
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
            .child(self.title.clone());

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
        let models = Model::get_models();
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

        let default_model = models.get(0).unwrap();
        let model = storage
            .get(StorageKey::AssistantModel)
            .map(|value| serde_json::from_str::<Model>(&value))
            .transpose() // converts Option<Result<T, E>> to Result<Option<T>, E>
            .map(|opt| opt.unwrap_or(default_model.clone()))
            .unwrap_or_else(|err| {
                println!("Error parsing model: {:?}", err);
                default_model.clone()
            });

        let api_key = (match model.provider {
            ModelProvider::Anthropic => storage.get(StorageKey::AnthropicApiKey),
            ModelProvider::OpenAI => storage.get(StorageKey::OpenAiApiKey),
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
                    ModelProvider::OpenAI => StorageKey::OpenAiApiKey,
                };

                let _ = storage.set(storage_key, api_key.into());
            }
        })
        .detach();

        let model_index = models.iter().position(|item| item.name == model.name);
        let model_dropdown = cx.new(|cx| {
            Dropdown::new("model-dropdown", models, model_index, window, cx)
                .placeholder("Select Model")
        });

        cx.subscribe(&model_dropdown, |this, _dropdown, event, cx| {
            let DropdownEvent::Confirm(value) = event;

            if let Some(model) = value {
                // Save model to storage
                let storage = cx.global::<Storage>();
                let _ = storage.set(
                    StorageKey::AssistantModel,
                    serde_json::to_string(&model).unwrap(),
                );

                this.provider = model.provider.clone();
                this.api_key_input.update(cx, |input, _cx| {
                    let placeholder = match model.provider {
                        ModelProvider::Anthropic => "Enter Anthropic API Key",
                        ModelProvider::OpenAI => "Enter OpenAI API Key",
                    };

                    input.set_placeholder(placeholder);
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
