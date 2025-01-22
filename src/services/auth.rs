use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use gpui::{AppContext, AsyncWindowContext, Global};
use rand::Rng;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::time::Duration;
use tiny_http::Server;
use url::Url;

use crate::errors::AuthError;

/*
 * Auth service that uses Supabase Auth API as the backend.
 */
#[derive(Clone)]
pub struct Auth {
    base_url: String,
    callback_url: String,
    client: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct AccessToken {
    pub access_token: String,
    expires_in: u64,
    pub expires_at: u64,
    pub refresh_token: String,
}

#[derive(Clone, Debug)]
pub struct User {
    id: String,
    email: String,
    avatar_url: String,
    full_name: String,
}

impl Global for Auth {}

impl Auth {
    pub fn init(cx: &mut AppContext) {
        let supabase_public_url = env!("SUPABASE_PUBLIC_URL");
        let supabase_public_anon_key = env!("SUPABASE_PUBLIC_ANON_KEY");

        if supabase_public_url.is_empty() || supabase_public_anon_key.is_empty() {
            // TODO: set state error
        }

        let mut headers = HeaderMap::new();

        headers.insert(
            "apikey",
            HeaderValue::from_str(supabase_public_anon_key).unwrap(),
        );
        headers.insert("X-Client-Info", HeaderValue::from_static("cmdi-rs/0.1.0"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        let auth = Auth {
            base_url: format!("{}/auth/v1", supabase_public_url),
            callback_url: "http://127.0.0.1:3100/auth/callback".to_string(),
            client,
        };

        cx.set_global(auth);
    }

    /**
     * Log in a user using magiclink.
     *
     * If the `{{ .ConfirmationURL }}` variable is specified in the email template, a magiclink will be sent.
     * If the `{{ .Token }}` variable is specified in the email template, an OTP will be sent.
     *
     * This method uses PKCE.
     */
    pub async fn login_with_email(
        &self,
        cx: &AsyncWindowContext,
        email: &str,
    ) -> Result<(AccessToken, User)> {
        let background = cx.background_executor().clone();
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);

        let url = format!("{}/otp?redirect_to={}", self.base_url, self.callback_url);

        let payload = serde_json::json!({
            "email": email.to_owned(),
            "create_user": true,
            "code_challenge": code_challenge,
            "code_challenge_method": "s256",
        });

        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(|original_err| AuthError::EmailLoginRequestError(original_err))?;

        // Setup local server to receive the request from the browser
        let server_addr = "127.0.0.1:3100".to_owned(); // TODO: Derive from callback URL
        let server = Server::http(server_addr.clone()).expect("Failed to start server");

        let code = background
            .spawn(async move {
                for _ in 0..100 {
                    if let Some(req) = server
                        .recv_timeout(Duration::from_secs(1))
                        .map_err(|_| AuthError::EmailLoginTimeoutError)?
                    {
                        let base_url = format!("http://{}", server_addr);
                        let full_url = format!("{}{}", base_url, req.url());

                        // Parse URL to extract query parameters
                        match Url::parse(&full_url) {
                            Ok(parsed_url) => {
                                let mut code = None;

                                for (key, value) in parsed_url.query_pairs() {
                                    if key == "code" {
                                        code = Some(value);
                                    }
                                }

                                let _ = req.respond(
                                    tiny_http::Response::empty(302).with_header(
                                        tiny_http::Header::from_bytes(
                                            &b"Location"[..],
                                            "https://www.cmdi.app/auth/success".as_bytes(),
                                        )
                                        .unwrap(),
                                    ),
                                );

                                return match code {
                                    Some(value) => Ok(value.into_owned()),
                                    None => Err(AuthError::EmailLoginNoAuthCode),
                                };
                            }
                            Err(e) => return Err(AuthError::EmailLoginParseError(e)),
                        }
                    }
                }

                Err(AuthError::EmailLoginTimeoutError)
            })
            .await?;

        let credentials = self.exchange_code_for_token(&code, &code_verifier).await?;

        Ok(credentials)
    }

    async fn exchange_code_for_token(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<(AccessToken, User)> {
        let url = format!("{}/token?grant_type=pkce", self.base_url);

        let payload = serde_json::json!({
            "auth_code": code,
            "code_verifier": code_verifier,
        });

        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(|original_err| AuthError::EmailLoginCodeError(original_err))?;

        let data = response.json::<serde_json::Value>().await?;

        let access_token = data.get("access_token");
        let user = data.get("user");

        if access_token.is_none() || user.is_none() {
            return Err(AuthError::EmailLoginInvalidPayload.into());
        }

        let token = AccessToken {
            access_token: access_token.unwrap().as_str().unwrap().to_owned(),
            expires_in: data.get("expires_in").unwrap().as_u64().unwrap(),
            expires_at: data.get("expires_at").unwrap().as_u64().unwrap(),
            refresh_token: data
                .get("refresh_token")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        };

        let user_metadata = user.unwrap().get("user_metadata").unwrap();

        let user = User {
            id: unwrap_value_to_string(user.unwrap().get("id")),
            email: unwrap_value_to_string(user.unwrap().get("email")),
            avatar_url: unwrap_value_to_string(user_metadata.get("avatar_url")),
            full_name: unwrap_value_to_string(user_metadata.get("full_name")),
        };

        Ok((token, user))
    }
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

fn unwrap_value_to_string(value: Option<&Value>) -> String {
    value.unwrap().as_str().unwrap().to_owned()
}

// Generate code verifier (random string between 43-128 chars)
fn generate_code_verifier() -> String {
    let mut rng = rand::thread_rng();
    let length = 56; // between 43-128
    let code_verifier: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    code_verifier
}

// Generate code challenge from verifier
fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = hasher.finalize();

    URL_SAFE_NO_PAD.encode(challenge)
}
