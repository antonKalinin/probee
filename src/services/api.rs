use anyhow::Result;
use gpui::{App, AsyncApp, Global};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use std::env;

use crate::errors::ApiError;
use crate::services::Auth;

/*
 * Api service that uses Supabase REST API as the backend.
 */
#[derive(Clone)]
pub struct Api {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub provider: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AssistantConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub model: Model,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct GetAssistantsResponse {
    assistants: Vec<AssistantConfig>,
}

impl Api {
    pub fn init(cx: &mut App) {
        let supabase_public_url = env!("SUPABASE_PUBLIC_URL");
        let supabase_public_key = env!("SUPABASE_PUBLIC_ANON_KEY");

        if supabase_public_url.is_empty() || supabase_public_key.is_empty() {
            // TODO: set state error
        }

        let mut headers = HeaderMap::new();
        let api_key_header = HeaderValue::from_str(supabase_public_key).unwrap();

        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("apikey", api_key_header);
        headers.insert("X-Client-Info", HeaderValue::from_static("cmdi-rs/0.1.0"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        cx.set_global(Api {
            client,
            base_url: supabase_public_url.to_owned(),
        });
    }

    pub async fn get_assistants(&self, cx: &mut AsyncApp) -> Result<Vec<AssistantConfig>> {
        // Returns public assistants for authenticated users and personal assistants for their authors.
        let url = format!("{}{}", self.base_url, "/rest/v1/assistants");

        let access_token = Auth::get_access_token_async(cx).unwrap_or("".into());
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|original_err| ApiError::RequestError(original_err))?;

        let status = response.status();

        if !status.is_success() {
            return Ok(vec![]);
        }

        let assistants = response
            .json::<Vec<AssistantConfig>>()
            .await
            .map_err(|original_err| ApiError::DecodingError(original_err))?;

        Ok(assistants)
    }
}

impl Global for Api {}
