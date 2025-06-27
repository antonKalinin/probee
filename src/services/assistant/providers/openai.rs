use anyhow::Result;
use futures::StreamExt;
use gpui::App;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;

use crate::errors::*;
use crate::services::assistant::{AssistantProviderClient, Model};
use crate::services::storage::{Storage, StorageKey};
use crate::state::app_state::set_error;

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    max_tokens: i32,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    delta: ChoiceDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChoiceDelta {
    role: Option<String>,
    content: Option<String>,
}

#[derive(Clone, Debug)]
pub struct OpenAIProviderClient {
    client: Client,
    api_key: String,
}

impl OpenAIProviderClient {
    pub fn new(cx: &mut App) -> Self {
        let storage = cx.global::<Storage>();
        let api_key = storage.get(StorageKey::OpenAiApiKey).unwrap_or_default();

        if api_key.is_empty() {
            set_error(
                cx,
                Some(AssistantError::MissingProviederApiKey(String::from("OpenAI")).into()),
            );
        }

        let mut headers = HeaderMap::new();

        // headers.insert(
        //     "Authorization",
        //     HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        // );
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client, api_key }
    }
}

type ResultStream = ReceiverStream<String>;

#[async_trait::async_trait]
impl AssistantProviderClient for OpenAIProviderClient {
    fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key.clone();
    }

    async fn generate_response(
        &self,
        model: Model,
        system_prompt: String,
        user_input: String,
    ) -> Result<ResultStream> {
        let mut messages = Vec::new();

        // Add system message if provided
        if !system_prompt.is_empty() {
            messages.push(OpenAIMessage {
                role: "system".to_owned(),
                content: system_prompt,
            });
        }

        // Add user message
        messages.push(OpenAIMessage {
            role: "user".to_owned(),
            content: user_input,
        });

        let request = OpenAIRequest {
            model: model.name.clone(),
            messages,
            temperature: 0.2,
            max_tokens: 1024,
            stream: true,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(OutputError::AssistantRequestError(error_text).into());
        }

        let (tx, rx) = channel(1);
        let output_stream = ReceiverStream::new(rx);
        let mut response_stream = response.bytes_stream();

        tokio::spawn(async move {
            while let Some(chunk) = response_stream.next().await {
                if chunk.is_err() {
                    continue;
                }
                let chunk = chunk.unwrap();
                let chunk_str = String::from_utf8_lossy(&chunk);

                // Process each line in the chunk
                for line in chunk_str.lines() {
                    // OpenAI streaming format: "data: {...}"
                    if line.starts_with("data: ") {
                        let data = &line["data: ".len()..];

                        // Skip empty data or [DONE] marker
                        if data.trim().is_empty() || data.trim() == "[DONE]" {
                            continue;
                        }

                        if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
                            for choice in response.choices {
                                if let Some(content) = choice.delta.content {
                                    if !content.is_empty() {
                                        if tx.send(content).await.is_err() {
                                            return;
                                        }
                                    }
                                }

                                // Break on finish
                                if choice.finish_reason.is_some() {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(output_stream)
    }

    fn box_clone(&self) -> Box<dyn AssistantProviderClient> {
        Box::new(self.clone())
    }
}
