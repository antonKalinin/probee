#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ChangeAssistant(String),
    CopyOutput,
    ClearOutput,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    InputChanged(String),
    AssistantChanged(String),
}
