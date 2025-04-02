#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    SelectAssistant(String),
    ToggleAssistantLibrary,
    CopyOutput,
    ClearOutput,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    AssistantChanged(String),
    Authenticated,
    InputChanged(String),
    OpenSettings,
}
