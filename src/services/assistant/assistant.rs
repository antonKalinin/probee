use anyhow::Result;
use gpui::{App, BorrowAppContext, Global, SharedString};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

use crate::errors::*;
use crate::services::storage::{Storage, StorageKey};

use super::providers::*;

type ResultStream = ReceiverStream<String>;

#[async_trait::async_trait]
pub trait AssistantProviderClient {
    async fn generate_response(
        &self,
        model: Model,
        system_prompt: String,
        user_input: String,
    ) -> Result<ResultStream>;

    fn set_api_key(&mut self, api_key: String);

    fn box_clone(&self) -> Box<dyn AssistantProviderClient>;
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum ModelProvider {
    Anthropic,
    OpenAI,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Model {
    pub title: SharedString,
    pub name: String,
    pub provider: ModelProvider,
}

impl Model {
    pub fn new(title: impl Into<SharedString>, name: String, provider: ModelProvider) -> Self {
        Self {
            title: title.into(),
            name: name.into(),
            provider,
        }
    }

    pub fn get_models() -> Vec<Self> {
        vec![
            Self::new(
                "Claude 4 Opus",
                "claude-opus-4-20250514".to_string(),
                ModelProvider::Anthropic,
            ),
            Self::new(
                "Claude 4 Sonnet",
                "claude-sonnet-4-20250514".to_string(),
                ModelProvider::Anthropic,
            ),
            Self::new(
                "Claude 3.7",
                "claude-3-7-sonnet-20250219".to_string(),
                ModelProvider::Anthropic,
            ),
            Self::new(
                "Claude 3.5 Sonnet",
                "claude-3-5-sonnet-20241022".into(),
                ModelProvider::Anthropic,
            ),
            Self::new(
                "Claude 3 Haiku",
                "claude-3-haiku-20240307".into(),
                ModelProvider::Anthropic,
            ),
            Self::new(
                "GPT-4.1 Nano",
                "gpt-4.1-nano-2025-04-14".to_string(),
                ModelProvider::OpenAI,
            ),
            Self::new(
                "GPT-4.0 Turbo",
                "gpt-4-turbo-2024-04-09".to_string(),
                ModelProvider::OpenAI,
            ),
            Self::new(
                "o4 Mini",
                "o4-mini-2025-04-16".to_string(),
                ModelProvider::OpenAI,
            ),
        ]
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Prompt {
    pub id: String,
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub temperature: f32,
    pub readonly: Option<bool>,
    pub updated_at: String,
    pub created_at: String,
}

impl Prompt {
    pub fn new(name: String, message: String) -> Self {
        let now = OffsetDateTime::now_utc();
        let now_iso = now.format(&Rfc3339).unwrap();

        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: "Select text on the screen and run this prompt".into(),
            system_message: message,
            temperature: 0.2,
            readonly: None,
            created_at: now_iso.clone(),
            updated_at: now_iso.clone(),
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.system_message = message;
        self
    }

    pub fn set_readonly(&mut self, readonly: bool) -> &mut Self {
        self.readonly = Some(readonly);
        self
    }
}

pub struct Assistant {
    model: Option<Model>,
    prompt: Option<Prompt>,
    provider_client: Option<Box<dyn AssistantProviderClient>>,
}

impl Clone for Assistant {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
            prompt: self.prompt.clone(),
            provider_client: self.provider_client.as_ref().map(|p| p.box_clone()),
        }
    }
}

impl Assistant {
    pub fn init(cx: &mut App) {
        let storage = cx.global::<Storage>();
        let default_model = Model::get_models().get(0).unwrap().clone();
        let model: Option<Model> = storage
            .get(StorageKey::AssistantModel)
            .map(|model_str| serde_json::from_str(&model_str).unwrap_or(default_model));

        let provider_client: Option<Box<dyn AssistantProviderClient>> = match model.clone() {
            Some(model) => match model.provider {
                ModelProvider::Anthropic => Some(Box::new(AnthropicProviderClient::new(cx))),
                ModelProvider::OpenAI => Some(Box::new(OpenAIProviderClient::new(cx))),
            },
            None => None,
        };

        cx.set_global(Assistant {
            model,
            prompt: None,
            provider_client,
        });

        cx.update_global(|storage: &mut Storage, _cx| {
            storage.subscribe(|key, value, cx| match key {
                StorageKey::AnthropicApiKey | StorageKey::OpenAiApiKey => {
                    let provider_client = &mut cx.global_mut::<Assistant>().provider_client;

                    if let Some(client) = provider_client {
                        client.set_api_key(value);
                    }
                }
                _ => {}
            });
        });
    }

    pub fn set_model(&mut self, model: Model, cx: &mut App) {
        self.provider_client = match model.provider {
            ModelProvider::Anthropic => Some(Box::new(AnthropicProviderClient::new(cx))),
            ModelProvider::OpenAI => Some(Box::new(OpenAIProviderClient::new(cx))),
        };

        self.model = Some(model);
    }

    pub fn set_prompt(&mut self, prompt: Prompt) {
        self.prompt = Some(prompt);
    }

    pub async fn generate_response(&mut self, input: String) -> Result<ResultStream> {
        if self.provider_client.is_none() || self.model.is_none() {
            return Err(AssistantError::MissingProviderClient.into());
        }

        if self.prompt.is_none() {
            return Err(AssistantError::MissingPrompt.into());
        }

        let system_prompt = self.prompt.as_ref().unwrap().system_message.clone();
        let provider_client = self.provider_client.as_ref().unwrap();
        let model = self.model.as_ref().unwrap().clone();

        provider_client
            .generate_response(model, system_prompt, input.to_owned())
            .await
    }
}

impl Global for Assistant {}
