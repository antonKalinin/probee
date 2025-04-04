use anyhow::Error;
use gpui::App;

pub trait ErrorState {
    fn get_error(&self) -> Option<&Error>;
}

pub trait ErrorStateController {
    fn set_error(&self, cx: &mut App, error: Option<Error>);
}
