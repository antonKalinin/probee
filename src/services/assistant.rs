use anyhow::Result;
use gpui::{AppContext, Global};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::api::AssistantConfig;
use crate::errors::*;
use crate::state::*;

/**
 * By design LLM service should be agnostic to LLM provider.
 * Currently as a short term solution we use Anthropic API.
 * In the future, we can request own API which is proxy to multiple LLM providers.
*/

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    system: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Deserialize, Debug)]
struct Content {
    text: String,
}

#[derive(Deserialize, Debug)]
struct AnthropicResponse {
    content: Vec<Content>,
    model: String,
    role: String,
}

#[derive(Clone)]
pub struct Assistant {
    client: Client,
}

impl Assistant {
    pub fn init(cx: &mut AppContext) {
        let api_key = env!("ANTHROPIC_API_KEY");

        if api_key.is_empty() {
            // set state error
        }

        let mut headers = HeaderMap::new();

        headers.insert("x-api-key", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        cx.set_global(Assistant { client });
    }

    // fn resolve_system_prompt(&self, config: AssistantConfig) -> Result<String> {
    //     let system_prompt = config
    //         .messages
    //         .iter()
    //         .find(|message| message.role == "system");

    //     if let Some(system_prompt) = system_prompt {
    //         return Ok(system_prompt.content.clone());
    //     } else {
    //         return Err(InputError::MissingSystemPromptError.into());
    //     }
    // }

    // pub async fn request(&self, input: &str) -> Result<String> {
    //     let config = get_active_assistant();
    //     let system_prompt = self.resolve_system_prompt(config)?;
    //     let request = AnthropicRequest {
    //         model: "claude-3-5-sonnet-20241022".to_owned(),
    //         system: system_prompt,
    //         temperature: 0.2,
    //         messages: vec![Message {
    //             role: "user".to_owned(),
    //             content: input.to_owned(),
    //         }],
    //         max_tokens: 1024,
    //     };

    //     let response = self
    //         .client
    //         .post("https://api.anthropic.com/v1/messages")
    //         .json(&request)
    //         .send()
    //         .await?;

    //     if !response.status().is_success() {
    //         let error_text = response.text().await?;
    //         return Err(OutputError::AssistantRequestError(error_text).into());
    //     }

    //     let anthropic_response: AnthropicResponse = response.json().await?;

    //     if let Some(first_content) = anthropic_response.content.first() {
    //         return Ok(first_content.text.to_owned());
    //     }

    //     Err(OutputError::EmptyResponseError.into())
    // }
}

impl Global for Assistant {}
