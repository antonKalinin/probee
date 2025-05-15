#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ToggleAssistantLibrary,
    CopyOutput,
    ClearOutput,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    AssistantChanged(String),
    Authenticated,
    InputChanged(String),
    OpenSettings,
}
