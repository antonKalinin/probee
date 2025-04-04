#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
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

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsEvent {
    SettingsTabSelected,
}
