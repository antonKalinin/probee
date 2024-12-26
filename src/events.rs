use crate::state::ActiveView;

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ChangeActiveView(ActiveView),
    ChangeAssistant(String),
    CopyOutput,
    CloseWindow,
    HideWindow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    InputUpdated(String),
}
