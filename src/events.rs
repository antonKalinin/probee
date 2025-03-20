#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ChangeAssistant(String),
    CopyOutput,
    ClearOutput,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    AssistantChanged(String),
    Authenticated,
    InputChanged(String),
    VisibilityChanged(bool),
}
