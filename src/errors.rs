use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to decode response from API\n{0}")]
    DecodingError(reqwest::Error),
    #[error("Request to API failed\n{0}")]
    RequestError(reqwest::Error),
}

#[derive(Error, Debug)]
pub enum AssistantError {
    #[error("Assistant config is missing\nWe know about this issue and are working on it.")]
    MissingConfig,
    #[error("Assistant provider is missing\nWe know about this issue and are working on it.")]
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
    #[error("Error while getting clipboard content\nPlease try again.")]
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
