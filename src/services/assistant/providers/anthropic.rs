use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::errors::*;
use crate::services::assistant::AssistantProvider;

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

#[derive(Clone, Debug)]
pub struct AnthropicProvider {
    client: Client,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        let api_key = env!("ANTHROPIC_API_KEY");

        if api_key.is_empty() {
            // set state error
        }

        let mut headers = HeaderMap::new();

        headers.insert("x-api-key", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client }
    }
}

#[async_trait::async_trait]
impl AssistantProvider for AnthropicProvider {
    async fn generate_response(&self, system_prompt: String, user_input: String) -> Result<String> {
        let request = AnthropicRequest {
            model: "claude-3-5-sonnet-20241022".to_owned(),
            system: system_prompt,
            temperature: 0.2,
            messages: vec![Message {
                role: "user".to_owned(),
                content: user_input,
            }],
            max_tokens: 1024,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(OutputError::AssistantRequestError(error_text).into());
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        if let Some(first_content) = anthropic_response.content.first() {
            return Ok(first_content.text.to_owned());
        }

        Err(OutputError::EmptyResponseError.into())
    }

    fn box_clone(&self) -> Box<dyn AssistantProvider> {
        Box::new(self.clone())
    }
}
