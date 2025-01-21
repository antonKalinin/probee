use anyhow::Result;
use gpui::{AppContext, Global};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use std::env;

use crate::errors::ApiError;

#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Provider {
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

    pub async fn get_public_assistants(&self) -> Result<Vec<AssistantConfig>> {
        let url = format!("{}{}", self.base_url, "/v1/assistants");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|original_err| ApiError::RequestError(original_err))?;

        // TODO: Check response status code and return error if it's not 200

        let decoded_response = response
            .json::<GetAssistantsResponse>()
            .await
            .map_err(|original_err| ApiError::DecodingError(original_err))?;

        let assistants = decoded_response.assistants;

        Ok(assistants)
    }
}

impl Global for Api {}
