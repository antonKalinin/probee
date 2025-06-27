use gpui::*;

use crate::assistant::{Assistant, Model, ModelProvider, Prompt};
use crate::state::settings_state::*;
use crate::storage::{Storage, StorageKey};
use crate::ui::{
    ActiveTheme, Button, Dropdown, DropdownEvent, DropdownItem, Icon, IconName, InputEvent,
    InputState, PromptEditorView, Root, Sizable as _, TextInput, Theme,
};
use crate::utils::prompt_window_options;

use super::components::PromptList;

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
    api_key_input: Entity<InputState>,
    model_dropdown: Entity<Dropdown<Vec<Model>>>,
    prompt_list: Entity<PromptList>,

    provider: ModelProvider,
    prompt_window_handle: Option<WindowHandle<Root>>,
}

impl AssistantSettingsView {
    pub fn new(state: &Entity<SettingsState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let storage = cx.global::<Storage>();
        let models = Model::get_models();

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
            InputState::new(window, cx)
                .placeholder("Enter Model Provider API Key")
                .default_value(api_key)
        });

        cx.subscribe(&api_key_input, |this, _input, event, cx| {
            if let InputEvent::Change(api_key) = event {
                let storage_key = match this.provider {
                    ModelProvider::Anthropic => StorageKey::AnthropicApiKey,
                    ModelProvider::OpenAI => StorageKey::OpenAiApiKey,
                };

                cx.update_global(|storage: &mut Storage, cx| {
                    storage
                        .set_notify(storage_key, api_key.to_string(), cx)
                        .ok();
                });
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

                let _api_key = match model.provider {
                    ModelProvider::Anthropic => storage.get(StorageKey::AnthropicApiKey),
                    ModelProvider::OpenAI => storage.get(StorageKey::OpenAiApiKey),
                }
                .unwrap_or(String::from(""));

                cx.update_global(|assistant: &mut Assistant, cx| {
                    assistant.set_model(model.clone(), cx);
                });

                this.provider = model.provider.clone();

                cx.notify();
            }
        })
        .detach();

        let handle_select_prompt = cx.listener(|this, prompt: &Prompt, _window, cx| {
            let handle_close = cx.listener(|this, _ok, window, cx| {
                window.remove_window();
                this.prompt_window_handle = None;
                cx.notify();
            });

            if this.prompt_window_handle.is_some() {
                // Update the existing prompt window
                let window_handle = this.prompt_window_handle.unwrap();
                let _ = cx.update_window(window_handle.into(), |_this, window, cx| {
                    window.replace_root(cx, |window, cx| {
                        let view = cx.new(|cx| {
                            PromptEditorView::new(Some(prompt.clone()), handle_close, window, cx)
                        });

                        Root::new(view.into(), window, cx)
                    });
                });

                return;
            };

            let window_options = prompt_window_options(cx);
            let window_handle = cx
                .open_window(window_options, |window, cx| {
                    PromptEditorView::build(Some(prompt.clone()), handle_close, window, cx)
                })
                .ok();

            this.prompt_window_handle = window_handle;
        });

        let prompt_list = cx.new(|cx| PromptList::new(state, handle_select_prompt, window, cx));

        AssistantSettingsView {
            api_key_input,
            model_dropdown,
            prompt_list,

            provider: model.provider.clone(),
            prompt_window_handle: None,
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
                .on_click(
                    cx.listener(|this, _event, _window, cx: &mut Context<Self>| {
                        let handle_close = cx.listener(|this, _, window, cx| {
                            window.remove_window();
                            this.prompt_window_handle = None;
                            cx.notify();
                        });

                        if this.prompt_window_handle.is_some() {
                            // Update the existing prompt window
                            let window_handle = this.prompt_window_handle.unwrap();

                            let _ = cx.update_window(window_handle.into(), |_, window, cx| {
                                window.replace_root(cx, |window, cx| {
                                    PromptEditorView::new(None, handle_close, window, cx)
                                });
                            });

                            return;
                        }

                        let window_options = prompt_window_options(cx);
                        let window_handle = cx
                            .open_window(window_options, |window, cx| {
                                PromptEditorView::build(None, handle_close, window, cx)
                            })
                            .ok();

                        this.prompt_window_handle = window_handle;
                    }),
                ),
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
                        value().child(TextInput::new(&self.api_key_input)),
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
                    .children(vec![label(""), value().child(create_prompt_button)])
                    .mt_8(),
            )
            .into_any_element()
    }
}
