use crate::assistant::AssistMode;

#[derive(Clone)]
pub enum UiEvent {
    ModeChanged(AssistMode),
    ClearOutput,
    CopyOutput,
}
