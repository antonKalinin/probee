use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Can't call platform API to get screen selection")]
    SelectionApiError,
    #[error("No text selected")]
    EmptySelectionError,
    #[error("Oh... Something isn't good here.")]
    UnknownError,
}

#[derive(Error, Debug)]
pub enum AssistantError {}
