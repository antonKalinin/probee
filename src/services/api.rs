use anyhow::Result;
use gpui::{App, AsyncApp, Global};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use std::env;

use crate::errors::ApiError;
use crate::services::Prompt;

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

#[derive(Deserialize, Debug)]
pub struct GetAssistantsResponse {
    prompts: Vec<Prompt>,
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
        headers.insert("X-Client-Info", HeaderValue::from_static("probee-rs/0.1.0"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        cx.set_global(Api {
            client,
            base_url: supabase_public_url.to_owned(),
        });
    }

    pub async fn get_prompts(&self, _cx: &mut AsyncApp) -> Result<Vec<Prompt>> {
        let url = format!("{}{}", self.base_url, "/rest/v1/prompts");

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|original_err| ApiError::RequestError(original_err))?;

        let status = response.status();

        if !status.is_success() {
            return Ok(vec![]);
        }

        let mut prompts = response
            .json::<Vec<Prompt>>()
            .await
            .map_err(|original_err| ApiError::DecodingError(original_err))?;

        prompts.iter_mut().for_each(|prompt| {
            prompt.set_readonly(true);
        });

        Ok(prompts)
    }
}

impl Global for Api {}
