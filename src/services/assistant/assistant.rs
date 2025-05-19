use anyhow::Result;
use gpui::{App, Global, SharedString};
use serde::{Deserialize, Serialize};
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
        system_prompt: String,
        user_input: String,
    ) -> Result<ResultStream>;

    fn box_clone(&self) -> Box<dyn AssistantProviderClient>;
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum ModelProvider {
    Anthropic,
    OpenAI,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Model {
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

#[derive(Clone, Default)]
pub struct Prompt {
    id: String,
    name: SharedString,
    text: SharedString,
    created_at: String,
    updated_at: String,
}

impl Prompt {
    pub fn new(name: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            text: text.into(),
            created_at: "2025-05-13T14:25:30.123Z".into(),
            updated_at: "2025-05-13T14:25:30.123Z".into(),
        }
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
        let model: Option<Model> = storage
            .get(StorageKey::AssistantModel)
            .map(|model_str| serde_json::from_str(&model_str).unwrap());

        let provider_client: Option<Box<dyn AssistantProviderClient>> = match model.clone() {
            Some(model) => match model.provider {
                ModelProvider::Anthropic => Some(Box::new(AnthropicProviderClient::new(cx))),
                _ => Some(Box::new(AnthropicProviderClient::new(cx))),
            },
            None => None,
        };

        cx.set_global(Assistant {
            model,
            prompt: None,
            provider_client,
        });
    }

    pub fn set_model(&mut self, model: Model, cx: &mut App) {
        self.model = Some(model.clone());
        self.provider_client = match model.provider {
            ModelProvider::Anthropic => Some(Box::new(AnthropicProviderClient::new(cx))),
            _ => None,
        };
    }

    pub fn set_prompt(&mut self, prompt: Prompt) {
        self.prompt = Some(prompt);
    }

    pub async fn generate_response(&self, input: String) -> Result<ResultStream> {
        if self.provider_client.is_none() {
            return Err(AssistantError::MissingProviderClient.into());
        }

        if self.prompt.is_none() {
            return Err(AssistantError::MissingSystemPrompt.into());
        }

        let system_prompt = self.prompt.as_ref().unwrap().text.to_string();
        let provider_client = self.provider_client.as_ref().unwrap();

        provider_client
            .generate_response(system_prompt, input.to_owned())
            .await
    }
}

impl Global for Assistant {}
