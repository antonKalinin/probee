use crate::assistant::AssistMode;
use crate::state::ActiveView;

#[derive(Clone)]
pub enum UiEvent {
    ChangeActiveView(ActiveView),
    ClearOutput,
    CopyOutput,
    ChangeMode(AssistMode),
    CloseWindow,
    HideWindow,
}
