use anyhow::{Ok, Result};
use gpui::{App, Global};
use tokio_stream::wrappers::ReceiverStream;

use crate::api::AssistantConfig;
use crate::errors::*;

use super::providers::*;

type ResultStream = ReceiverStream<String>;

#[async_trait::async_trait]
pub trait AssistantProvider {
    async fn generate_response(
        &self,
        system_prompt: String,
        user_input: String,
    ) -> Result<ResultStream>;

    fn box_clone(&self) -> Box<dyn AssistantProvider>;
}

pub struct Assistant {
    config: Option<AssistantConfig>,
    provider: Option<Box<dyn AssistantProvider>>,
}

impl Clone for Assistant {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            provider: self.provider.as_ref().map(|p| p.box_clone()),
        }
    }
}

impl Assistant {
    fn resolve_system_prompt(&self) -> Result<String> {
        if self.config.is_none() {
            return Err(AssistantError::MissingConfig.into());
        }

        let config = self.config.as_ref().unwrap();
        let system_prompt = config
            .messages
            .iter()
            .find(|message| message.role == "system");

        if let Some(system_prompt) = system_prompt {
            return Ok(system_prompt.content.clone());
        } else {
            return Err(AssistantError::MissingSystemPrompt.into());
        }
    }

    pub fn init(cx: &mut App) {
        cx.set_global(Assistant {
            config: None,
            provider: None,
        });
    }

    pub fn set_config(&mut self, config: AssistantConfig) -> Result<()> {
        let provider_name = config.provider.name.clone();

        match provider_name.as_str() {
            "anthropic" => {
                self.config = Some(config);
                self.provider = Some(Box::new(AnthropicProvider::new()));
            }
            _ => {
                return Err(AssistantError::UnsupportedProvider(provider_name).into());
            }
        }

        Ok(())
    }

    pub async fn generate_response(&self, input: String) -> Result<ResultStream> {
        if self.provider.is_none() {
            return Err(AssistantError::MissingProvider.into());
        }

        let system_prompt = self.resolve_system_prompt()?;
        let provider = self.provider.as_ref().unwrap();

        provider
            .generate_response(system_prompt, input.to_owned())
            .await
    }
}

impl Global for Assistant {}
