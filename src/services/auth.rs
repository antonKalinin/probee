use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use gpui::{App, AsyncApp, Global};
use rand::Rng;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::time::Duration;
use tiny_http::Server;
use url::Url;

use crate::errors::AuthError;
use crate::services::Storage;

const STORAGE_USER_ID_KEY: &str = "user_id";
const STORAGE_ACCESS_TOKEN_KEY: &str = "access_token";
const STORAGE_REFRESH_TOKEN_KEY: &str = "refresh_token";
const STORAGE_ACCESS_TOKEN_EXPIRES_AT_KEY: &str = "access_token_expires_at";

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
    pub expires_at: u64,
    pub refresh_token: String,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub avatar_url: String,
    pub full_name: String,
}

impl Global for Auth {}

impl Auth {
    pub fn init(cx: &mut App) {
        let supabase_public_url = env!("SUPABASE_PUBLIC_URL");
        let supabase_public_key = env!("SUPABASE_PUBLIC_ANON_KEY");

        if supabase_public_url.is_empty() || supabase_public_key.is_empty() {
            // TODO: set state error
        }

        let mut headers = HeaderMap::new();

        headers.insert(
            "apikey",
            HeaderValue::from_str(supabase_public_key).unwrap(),
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

    pub fn get_access_token_async(cx: &mut AsyncApp) -> Option<String> {
        cx.read_global(|storage: &Storage, _cx| storage.get(STORAGE_ACCESS_TOKEN_KEY))
            .unwrap_or(None)
    }

    fn get_refresh_token(cx: &mut AsyncApp) -> Option<String> {
        cx.read_global(|storage: &Storage, _cx| storage.get(STORAGE_REFRESH_TOKEN_KEY))
            .unwrap_or(None)
    }

    fn reset_auth_data(cx: &mut AsyncApp) -> Result<()> {
        cx.read_global(|storage: &Storage, _cx| {
            let _ = storage.delete(STORAGE_USER_ID_KEY);
            let _ = storage.delete(STORAGE_ACCESS_TOKEN_KEY);
            let _ = storage.delete(STORAGE_REFRESH_TOKEN_KEY);
            let _ = storage.delete(STORAGE_ACCESS_TOKEN_EXPIRES_AT_KEY);
        })?;

        Ok(())
    }

    // fn set_token_to_store(cx: &mut AsyncApp, token: AccessToken) -> Result<()> {
    //     cx.read_global(|storage: &Storage, _cx| {
    //         let expires_at = token.expires_at.to_string();

    //         storage.set(STORAGE_ACCESS_TOKEN_KEY.into(), token.access_token)?;
    //         storage.set(STORAGE_REFRESH_TOKEN_KEY.into(), token.refresh_token)?;
    //         storage.set(STORAGE_ACCESS_TOKEN_EXPIRES_AT_KEY.into(), expires_at)?;

    //         Ok(())
    //     })?;

    //     Ok(())
    // }

    /**
     * Log in a user using magiclink.
     *
     * If the `{{ .ConfirmationURL }}` variable is specified in the email template, a magiclink will be sent.
     * If the `{{ .Token }}` variable is specified in the email template, an OTP will be sent.
     *
     * This method uses PKCE.
     */
    pub async fn login_with_email(&self, cx: &mut AsyncApp, email: &str) -> Result<User> {
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

        let _ = self
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

        let (token, user) = self.exchange_code_for_token(&code, &code_verifier).await?;
        let user_id = user.id.clone();

        let _: Result<()> = cx.read_global(|storage: &Storage, _cx| {
            let expires_at = token.expires_at.to_string();

            storage.set("user_id".into(), user_id)?;
            storage.set("access_token".into(), token.access_token)?;
            storage.set("refresh_token".into(), token.refresh_token)?;
            storage.set("access_token_expires_at".into(), expires_at)?;

            Ok(())
        })?;

        Ok(user)
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
            return Err(AuthError::EmailLoginInvalidPayloadError.into());
        }

        let token = AccessToken {
            access_token: access_token.unwrap().as_str().unwrap().to_owned(),
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

    pub async fn refresh_access_token(&self, cx: &mut AsyncApp) -> Result<User> {
        let refresh_token = Auth::get_refresh_token(cx);

        if refresh_token.is_none() {
            return Err(AuthError::NoTokenError.into());
        }

        let url = format!("{}/token?grant_type=refresh_token", self.base_url);

        let payload = serde_json::json!({
            "refresh_token": refresh_token,
        });

        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(|original_err| AuthError::RefreshTokenRequestError(original_err))?;

        let status = response.status();
        let data = response.json::<serde_json::Value>().await?;

        if !status.is_success() {
            let message = data
                .get("msg")
                .unwrap()
                .as_str()
                .unwrap_or("unknown reason of error")
                .into();

            return Err(AuthError::InvalidRefreshTokenError(message).into());
        }

        let access_token = data.get("access_token");
        let user = data.get("user");

        let token = AccessToken {
            access_token: access_token.unwrap().as_str().unwrap().to_owned(),
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

        let user_id = user.id.clone();

        let _: Result<()> = cx.read_global(|storage: &Storage, _cx| {
            let expires_at = token.expires_at.to_string();

            storage.set("user_id".into(), user_id)?;
            storage.set("access_token".into(), token.access_token)?;
            storage.set("refresh_token".into(), token.refresh_token)?;
            storage.set("access_token_expires_at".into(), expires_at)?;

            Ok(())
        })?;

        Ok(user)
    }

    pub async fn get_user(&self, cx: &mut AsyncApp) -> Result<User> {
        let access_token = Auth::get_access_token_async(cx);

        if access_token.is_none() {
            return Err(AuthError::NoTokenError.into());
        }

        let url = format!("{}/user", self.base_url);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", access_token.unwrap()))
            .send()
            .await
            .map_err(|original_err| AuthError::GetUserRequestError(original_err))?;

        let status = response.status();
        let data = response.json::<serde_json::Value>().await?;

        if !status.is_success() {
            if status == StatusCode::FORBIDDEN {
                let _ = Auth::reset_auth_data(cx);
            }

            let message = data
                .get("msg")
                .unwrap()
                .as_str()
                .unwrap_or("unknown reason of error")
                .into();

            return Err(AuthError::InvalidTokenError(message).into());
        }

        let user_metadata = data.get("user_metadata").unwrap();

        let user = User {
            id: unwrap_value_to_string(data.get("id")),
            email: unwrap_value_to_string(data.get("email")),
            avatar_url: unwrap_value_to_string(user_metadata.get("avatar_url")),
            full_name: unwrap_value_to_string(user_metadata.get("full_name")),
        };

        Ok(user)
    }

    pub async fn logout(&self, cx: &mut AsyncApp) -> Result<()> {
        let access_token = Auth::get_access_token_async(cx);

        if access_token.is_none() {
            return Ok(());
        }

        /*
         * Determines which sessions should be logged out.
         * Global means all sessions by this account.
         * Local means only this session.
         * Others means all other sessions except the current one.
         */
        let scope = "local";
        let url = format!("{}/logout?scope={}", self.base_url, scope);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", access_token.unwrap()))
            .send()
            .await
            .map_err(|original_err| AuthError::LogoutRequestError(original_err))?;

        let status = response.status();

        if !status.is_success() {
            let data = response.json::<serde_json::Value>().await?;
            let message = data
                .get("msg")
                .unwrap()
                .as_str()
                .unwrap_or("unknown reason of error")
                .into();

            return Err(AuthError::LogoutError(message).into());
        }

        let _: Result<()> = cx.read_global(|storage: &Storage, _cx| {
            storage.delete(STORAGE_USER_ID_KEY)?;
            storage.delete(STORAGE_ACCESS_TOKEN_KEY)?;
            storage.delete(STORAGE_REFRESH_TOKEN_KEY)?;
            storage.delete(STORAGE_ACCESS_TOKEN_EXPIRES_AT_KEY)?;

            Ok(())
        })?;

        Ok(())
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
