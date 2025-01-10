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
    #[error("Unsupported assistant provider\n{0}")]
    UnsupportedProvider(String),
}

#[allow(dead_code)]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    #[error("Error while getting screen selection\nPlease provide Cmdi Accessibility permissions: Settings -> Security & Privacy -> Accessibility")]
    SelectionApiError,
    #[error("Error while getting clipboard content\nPlease try again.")]
    ClipboardError,
    #[error("No text provided as input\nPlease copy some text and try again.")]
    EmptyTextInputError,
    #[error("Can't resolve assistnat intructions")]
    MissingSystemPromptError,
    #[error("We don't know what happened but it's not your fault\nPlease try again.")]
    UnknownError,
}

#[allow(dead_code)]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum OutputError {
    #[error("Request to assistant failed\n{0}")]
    AssistantRequestError(String),
    #[error("No response from assistant\nPlease try again.")]
    NoResponseError,
}
