use anyhow::{Error, Result};
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
    WordMorphology,
    PlainFinnish,
}

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

const TRANSLATE_MODE_PROMPT: &str = "\
You are a highly skilled translator with expertise in many languages. \
Your task is to identify the language of the text I provide and accurately \
translate it into the English while preserving the meaning, \
tone, and nuance of the original text. Please maintain proper grammar, spelling, \
punctuation including tabulation and new lines in the translated version. \
Please do not provide any additional information, titles, comments or context \
beyond the translated text.";

const TRANSLATE_WORD_BY_WORD: &str = "\
You are a highly skilled translator with expertise in many languages. \
Your task is to identify the language of the text I provide and accurately \
translate it into the English word by word. Treat each word as separate, without \
context of the whole sentence. \
Please provide the translation of each word in a new line in form:\
`original word - translation`. \
Please do not provide any additional information, titles, comments or context \
beyond the translated formatted text.";

const WORD_MORPHOLOGY: &str = "\
You are a highly skilled translator with expertise in many languages. \
Your task is to identify the language of the text I provide, take the first word, \
detect its part of speech, and provide all possible forms in the language of the text. \
Please provide the result in the following format: \
word (part of speech) - translation
form1 - translation1,
form2 - translation2,
...
formN - translationN
Please do not provide any additional information, titles, comments or context \
beyond the formatted result.";

const EXPLAIN_LIKE_IM_FIVE: &str = "\
You are a university professor with a specialization in the subject of the text I provide. \
Your task is to explain the text to a five-year-old child in a simple and understandable way. \
If it helps for clarity, you can use analogies, metaphors, or examples. \
";

const BASIC_FINNISH: &str = "\
You are a highly skilled translator with expertise in Finnish language. \
Your task is to rewrite the text I provide in a basic Finnish language: \
- Use simple words and short sentences. \
- Avoid complex grammar structures. \
Please do not provide any additional information, titles, comments or context \
beyond the adapted text.";

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

    fn resolve_system_prompt(&self, mode: AssistMode) -> String {
        match mode {
            AssistMode::Translate => TRANSLATE_MODE_PROMPT.to_string(),
            AssistMode::WordMorphology => WORD_MORPHOLOGY.to_string(),
            AssistMode::PlainFinnish => BASIC_FINNISH.to_string(),
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
            return Ok(first_content.text.to_string());
        }

        Err(Error::msg("No response from Claude"))
    }
}

impl Global for Assistant {}
