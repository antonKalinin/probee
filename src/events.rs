use crate::state::ActiveView;

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ChangeActiveView(ActiveView),
    ChangeAssistant(String),
    CopyOutput,
    CloseWindow,
    HideWindow,
    Login,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    InputChanged(String),
    AssistantChanged(String),
}
