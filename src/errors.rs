use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum InputError {
    #[error("Error while getting screen selection. Please provide Cmdi Accessibility permissions: Settings -> Security & Privacy -> Accessibility")]
    SelectionApiError,
    #[error("Error while getting clipboard content. Please try again.")]
    ClipboardError,
    #[error("No text provided as input. Please copy some text and try again.")]
    EmptyTextInputError,
    #[error("Oh... We don't know what happened but it's not your fault. Please try again.")]
    UnknownError,
}

#[derive(Error, Debug)]
pub enum AssistantError {}
