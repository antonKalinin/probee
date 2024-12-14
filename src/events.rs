use crate::assistant::AssistMode;

#[derive(Clone)]
pub enum UiEvent {
    AppButtonClicked,
    ClearOutput,
    CopyOutput,
    ChangeMode(AssistMode),
    CloseWindow,
    HideWindow,
}
