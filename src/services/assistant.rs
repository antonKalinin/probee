use anyhow::{Error, Result};
use dotenv::dotenv;
use gpui::{AppContext, Global};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum AssistMode {
    Translate,
    Explain,
    GrammarCorrect,
}

/**
 * By design LLM service should be agnostic to LLM provider.
 * Currently as a short term solution we use Anthropic API.
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

const TRANSLATE_MODE_PROMPT: &str = "
You are a highly skilled translator with expertise in many languages. \
Your task is to identify the language of the text I provide and accurately \
translate it into the English while preserving the meaning, \
tone, and nuance of the original text. Please maintain proper grammar, spelling, \
punctuation including tabulation and new lines in the translated version. \
Please do not provide any additional information, titles, comments or context \
beyond the translated text.";

const EXPLAIN_MODE_PROMPT: &str = "
Your task is to explain the text I provide in a clear and concise manner. \
Like I am five years old. If it is possible provide an example that explains \
or describes the give text or term in a simple way. Do not be too verbose, focus on \
the key points and make sure to use simple language.";

const GRAMMAR_CORRECT_MODE_PROMPT: &str = "
Your task is to take the text provided and rewrite it into a clear, grammatically \
correct version while preserving the original meaning as closely as possible. \
Correct any spelling mistakes, punctuation errors, verb tense issues, \
word choice problems, and other grammatical mistakes.";

impl Assistant {
    pub fn init(cx: &mut AppContext) {
        dotenv().ok();

        let api_key = env!("ANTHROPIC_API_KEY");

        if api_key.is_empty() {
            panic!("Assistant api key is missing");
        }

        let mut headers = HeaderMap::new();

        headers.insert("x-api-key", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        cx.set_global(Assistant { client });
    }

    fn resolve_system_prompt(&self, mode: AssistMode) -> String {
        match mode {
            AssistMode::Translate => TRANSLATE_MODE_PROMPT.to_string(),
            AssistMode::Explain => EXPLAIN_MODE_PROMPT.to_string(),
            AssistMode::GrammarCorrect => GRAMMAR_CORRECT_MODE_PROMPT.to_string(),
            _ => "What is weather today in Helsinki?".to_string(),
        }
    }

    pub async fn ask(&self, mode: AssistMode, input: &str) -> Result<String> {
        let system_prompt = self.resolve_system_prompt(mode);
        let request = AnthropicRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            system: system_prompt,
            temperature: 0.2,
            messages: vec![Message {
                role: "user".to_string(),
                content: input.to_string(),
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
            println!("API request failed: {:?}", error_text);
            return Err(Error::msg(format!("API request failed: {}", error_text)));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        if let Some(first_content) = anthropic_response.content.first() {
            println!("Claude's response: {}", first_content.text);

            return Ok(first_content.text.to_string());
        }

        Err(Error::msg("No response from Claude"))
    }
}

impl Global for Assistant {}
