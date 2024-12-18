use crate::assistant::AssistMode;
use crate::state::ActiveView;

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    ChangeActiveView(ActiveView),
    CopyOutput,
    ChangeMode(AssistMode),
    CloseWindow,
    HideWindow,
}
