use crate::state::ActiveView;

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ChangeAssistant(String),
    CopyOutput,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    InputChanged(String),
    EmailFormSubmitted(String),
    AssistantChanged(String),
}
