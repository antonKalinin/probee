use anyhow::Error;
use gpui::{App, AppContext, AsyncApp, BorrowAppContext, Entity, EventEmitter, Global};

use super::error_state::*;
use crate::events::AppEvent;
use crate::services::AssistantConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    AssistantView,
    LibraryView,
}

#[derive(Debug)]
pub struct AppState {
    pub active_assistant_id: Option<String>,
    pub active_view: AppView,
    pub assistants: Vec<AssistantConfig>,
    pub error: Option<Error>,
    pub input: Option<String>,
    pub output: String,
    pub blur_id: u16,

    pub focused: bool,
    pub loading: bool,
    pub visible: bool,
}

impl ErrorState for AppState {
    fn get_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }
}

impl EventEmitter<AppEvent> for AppState {}

#[derive(Clone)]
pub struct AppStateController {
    pub state: Entity<AppState>,
}

impl Global for AppStateController {}

impl ErrorStateController for AppStateController {
    fn set_error(&self, cx: &mut App, error: Option<Error>) {
        self.state.update(cx, |state, cx| {
            state.error = error;
            cx.notify();
        });
    }
}

impl AppStateController {
    pub fn init(cx: &mut App) {
        let state: Entity<AppState> = cx.new(|_cx| AppState {
            active_assistant_id: None,
            active_view: AppView::AssistantView,
            assistants: vec![],
            error: None,
            input: None,
            output: "".to_owned(),
            blur_id: 0,

            focused: false,
            loading: false,
            visible: true,
        });

        let global_state = AppStateController { state };

        cx.set_global(global_state);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut App), cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        })
    }

    pub fn update_async(f: impl FnOnce(&mut Self, &mut App), cx: &mut AsyncApp) {
        let _ = cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        });
    }

    pub fn set_active_assistant_id(&self, cx: &mut App, id: Option<String>) {
        self.state.update(cx, |state, cx| {
            state.active_assistant_id = id.clone();

            if let Some(id) = id {
                cx.emit(AppEvent::AssistantChanged(id));
            }

            cx.notify();
        });
    }

    pub fn set_assistants(&self, cx: &mut App, assistants: Vec<AssistantConfig>) {
        self.state.update(cx, |state, cx| {
            state.assistants = assistants;
            cx.notify();
        });
    }

    pub fn set_active_view(&self, cx: &mut App, view: AppView) {
        self.state.update(cx, |state, cx| {
            state.active_view = view.clone();
            state.error = None;

            cx.notify();
        });
    }

    pub fn set_input(&self, cx: &mut App, input: String) {
        self.state.update(cx, |state, cx| {
            state.input = Some(input.clone());
            cx.notify();
            cx.emit(AppEvent::InputChanged(input));
        });
    }

    pub fn set_output(&self, cx: &mut App, output: String) {
        self.state.update(cx, |state, cx| {
            state.output = output;
            cx.notify();
        });
    }

    pub fn append_output(&self, cx: &mut App, output: String) {
        self.state.update(cx, |state, cx| {
            state.output.push_str(&output);
            cx.notify();
        });
    }

    pub fn set_focused(&self, cx: &mut App, focused: bool) {
        self.state.update(cx, |state, cx| {
            if !focused {
                // Blur id increments every time app window loses focus.
                // If app won't get focus again in short time, window will be hidden.
                // Blur id helps to understand whatever app got focus again or not.
                state.blur_id = state.blur_id + 1;
                if state.blur_id > u16::MAX - 1 {
                    state.blur_id = 0;
                }
            }

            state.focused = focused;
            cx.notify();
        });
    }

    pub fn set_loading(&self, cx: &mut App, loading: bool) {
        self.state.update(cx, |model, cx| {
            model.loading = loading;
            cx.notify();
        });
    }

    pub fn set_visible(&self, cx: &mut App, visible: bool) {
        self.state.update(cx, |model, cx| {
            model.visible = visible;
            cx.notify();

            if visible {
                cx.activate(false);
            } else {
                cx.hide();
            }
        });
    }
}

/* Helper functions */

pub fn get_active_assistant(cx: &App) -> Option<AssistantConfig> {
    let state = cx.global::<AppStateController>().state.read(cx);

    match state.active_assistant_id.clone() {
        Some(id) => state
            .assistants
            .iter()
            .find(|assistant| assistant.id == id)
            .cloned(),
        None => None,
    }
}

pub fn get_focused(cx: &mut App) -> bool {
    let state = cx.global::<AppStateController>().state.read(cx);
    state.focused
}

pub fn get_blur_id(cx: &mut App) -> u16 {
    let state = cx.global::<AppStateController>().state.read(cx);
    state.blur_id
}

pub fn set_active_assistant_id(cx: &mut App, id: Option<String>) {
    AppStateController::update(|this, cx| this.set_active_assistant_id(cx, id), cx);
}

pub fn set_active_view(cx: &mut App, view: AppView) {
    AppStateController::update(|this, cx| this.set_active_view(cx, view), cx);
}

pub fn set_input(cx: &mut App, input: String) {
    AppStateController::update(|this, cx| this.set_input(cx, input), cx);
}

pub fn set_output(cx: &mut App, output: String) {
    AppStateController::update(|this, cx| this.set_output(cx, output), cx);
}

pub fn append_output_async(cx: &mut AsyncApp, output: String) {
    AppStateController::update_async(|this, cx| this.append_output(cx, output), cx);
}

pub fn set_loading(cx: &mut App, loading: bool) {
    AppStateController::update(|this, cx| this.set_loading(cx, loading), cx);
}

pub fn set_loading_async(cx: &mut AsyncApp, loading: bool) {
    AppStateController::update_async(|this, cx| this.set_loading(cx, loading), cx);
}

pub fn set_error(cx: &mut App, error: Option<Error>) {
    AppStateController::update(|this, cx| this.set_error(cx, error), cx);
}

pub fn set_error_async(cx: &mut AsyncApp, error: Option<Error>) {
    AppStateController::update_async(|this, cx| this.set_error(cx, error), cx);
}

pub fn set_focused(cx: &mut App, focused: bool) {
    AppStateController::update(|this, cx| this.set_focused(cx, focused), cx);
}

pub fn set_visible(cx: &mut App, visible: bool) {
    AppStateController::update(|this, cx| this.set_visible(cx, visible), cx);
}

pub fn toggle_visible(cx: &mut App) {
    AppStateController::update(
        |this, cx| {
            let state = this.state.read(cx);
            this.set_visible(cx, !state.visible);
        },
        cx,
    );
}
