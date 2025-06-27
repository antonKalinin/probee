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

use crate::services::assistant::{AssistantProviderClient, Model};
use crate::services::storage::{Storage, StorageKey};
use crate::{errors::*, state::settings_state::set_error};

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
    stream: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum StreamEvent {
    #[serde(rename = "message_start")]
    MessageStart,

    #[serde(rename = "content_block_start")]
    ContentBlockStart,

    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { delta: Delta, index: u32 },

    #[serde(rename = "content_block_stop")]
    ContentBlockStop,

    #[serde(rename = "message_delta")]
    MessageDelta,

    #[serde(rename = "message_stop")]
    MessageStop,

    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Deserialize)]
struct StreamMessage {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    content: Vec<ContentBlock>,
    model: String,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ContentBlockStart {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[serde(rename = "type")]
    delta_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct MessageDeltaContent {
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AnthropicProviderClient {
    client: Client,
    api_key: String,
}

impl AnthropicProviderClient {
    pub fn new(cx: &mut App) -> Self {
        let storage = cx.global::<Storage>();
        let api_key = storage.get(StorageKey::AnthropicApiKey).unwrap_or_default();

        if api_key.is_empty() {
            set_error(
                cx,
                Some(AssistantError::MissingProviederApiKey(String::from("Anthropic")).into()),
            );
        }

        let mut headers = HeaderMap::new();

        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client, api_key }
    }
}

type ResultStream = ReceiverStream<String>;

#[async_trait::async_trait]
impl AssistantProviderClient for AnthropicProviderClient {
    fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key.clone();
    }

    async fn generate_response(
        &self,
        model: Model,
        system_prompt: String,
        user_input: String,
    ) -> Result<ResultStream> {
        let request = AnthropicRequest {
            model: model.name.clone(),
            system: system_prompt,
            temperature: 0.2,
            messages: vec![Message {
                role: "user".to_owned(),
                content: user_input,
            }],
            max_tokens: 1024,
            stream: true,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", HeaderValue::from_str(&self.api_key).unwrap())
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

                // Split the chunk into individual events
                for line in chunk_str.lines() {
                    if line.starts_with("event: ") {
                        let _event_type = &line["event: ".len()..];
                        continue;
                    }

                    if line.starts_with("data: ") {
                        let event_data = &line["data: ".len()..];

                        // Skip empty events
                        if event_data.trim().is_empty() {
                            continue;
                        }

                        if let Ok(event) = serde_json::from_str::<StreamEvent>(event_data) {
                            match event {
                                // TODO: utilize index to maintain order of content blocks
                                // https://docs.anthropic.com/en/api/messages-streaming#event-types
                                StreamEvent::ContentBlockDelta { delta, index: _ } => {
                                    if tx.send(delta.text.clone()).await.is_err() {
                                        break;
                                    }
                                }
                                _ => { /*
                                        Ignore other events:
                                        - content_block_start
                                        - content_block_stop
                                        - message_start
                                        - message_stop
                                        - message_delta
                                        - ping
                                     */
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
