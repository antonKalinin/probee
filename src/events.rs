use crate::assistant::AssistMode;

#[derive(Clone)]
pub enum UiEvent {
    AppButtonClicked,
    ClearOutput,
    CopyOutput,
    ModeChanged(AssistMode),
    CloseWindow,
    HideWindow,
}
