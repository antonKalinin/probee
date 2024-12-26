use anyhow::Result;
use gpui::{AppContext, Global};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use std::env;

#[derive(Clone, Deserialize, Debug)]
struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Deserialize, Debug)]
struct Provider {
    pub name: String,
    pub model: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AssistantConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub provider: Provider,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct GetAssistantsResponse {
    assistants: Vec<AssistantConfig>,
}

#[derive(Clone)]
pub struct Api {
    client: Client,
    base_url: String,
}

impl Api {
    pub fn init(cx: &mut AppContext) {
        let api_url = env!("CMDI_API_URL");
        let mut headers = HeaderMap::new();

        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        cx.set_global(Api {
            client,
            base_url: api_url.to_owned(),
        });
    }

    pub async fn get_assistants(&self) -> Result<Vec<AssistantConfig>> {
        let url = format!("{}{}", self.base_url, "/v1/assistants");
        let response = self.client.get(url).send().await?;
        let response = response.json::<GetAssistantsResponse>().await?;

        let assistants = response.assistants;

        Ok(assistants)
    }
}

impl Global for Api {}
