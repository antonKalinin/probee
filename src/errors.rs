use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to decode response from API\n{0}")]
    DecodingError(reqwest::Error),
    #[error("Request to API failed\n{0}")]
    RequestError(reqwest::Error),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Error while logging in\n{0}")]
    EmailLoginRequestError(reqwest::Error),
    #[error("Error while loggin in\nTime out while waiting for magic link to be used. Please try to log in again.")]
    EmailLoginTimeoutError,
    #[error("Error while logging in\nCan't get auth code from the request. Please try again.")]
    EmailLoginNoAuthCode,
    #[error("Error while logging in\n{0}")]
    EmailLoginParseError(ParseError),
    #[error("Error while logging in\n{0}")]
    EmailLoginCodeError(reqwest::Error),
    #[error("Error while logging in\nAccess token or user is missing in the response. Please try again.")]
    EmailLoginInvalidPayload,
}

#[derive(Error, Debug)]
pub enum AssistantError {
    #[error("Assistant config is missing\nIt seems that you haven't selected any assistant. In case you have, please try again or restart the app.")]
    MissingConfig,
    #[error("Assistant provider is missing\nPlease try again or restart the app.")]
    MissingProvider,
    #[error("Can't resolve assistnat intructions")]
    MissingSystemPrompt,
    #[error("Unsupported assistant provider\n{0}")]
    UnsupportedProvider(String),
}

#[allow(dead_code)]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum InputError {
    #[error("Error while getting screen selection\nPlease provide Command I accessibility permissions:\n Settings -> Security & Privacy -> Accessibility")]
    AccessibilityPermissionsMissing,
    #[error("Error while getting screen selection\n{0}")]
    AppleScriptFailed(String),
    #[error("No text selected\nPlease select some text and try again.")]
    TextSelectionMissing,
    #[error("Error while getting clipboard content\nIt might be you don't have any text copied. Please copy some text and try again.")]
    ClipboardError,
    #[error("No text provided as input\nPlease copy some text and try again.")]
    EmptyTextInputError,
    #[error("Error while getting selected text\n{0}")]
    UnknownError(String),
}

#[allow(dead_code)]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum OutputError {
    #[error("Request to assistant failed\n{0}")]
    AssistantRequestError(String),
    #[error("No response from assistant\nPlease try again.")]
    NoResponseError,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Error while creating a storage\n")]
    StorageCreationError,
    #[error("Error while reading storage from disk\n{0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error\n{0}")]
    Json(#[from] serde_json::Error),
    #[error("Encryption error\n{0}")]
    Encryption(aes_gcm::Error),
    #[error("Decryption error")]
    Decryption,
    #[error("Error while setting a value\nPlease try to perform the operation again.")]
    SetError,
}
