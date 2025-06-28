use anyhow::Error;
use gpui::{
    App, AppContext, AsyncApp, BorrowAppContext, Entity, EventEmitter, Global, WindowHandle,
};

use super::error_state::*;
use crate::assistant::Prompt;
use crate::events::AppEvent;
use crate::storage::{Storage, StorageKey};
use crate::ui::Root;

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    AssistantView,
    LibraryView,
}

pub struct AppState {
    pub active_prompt_id: Option<String>,
    pub active_view: AppView,
    pub blur_id: u16,
    pub error: Option<Error>,
    pub input: Option<String>,
    pub output: String,
    pub prompts: Vec<Prompt>,

    pub focused: bool,
    pub loading: bool,
    pub visible: bool,
    pub assistant_processing: bool,
    pub settings_window_handle: Option<WindowHandle<Root>>,
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
        let storage = cx.global_mut::<Storage>();

        storage.subscribe(|key, value, cx| match key {
            StorageKey::Prompts => {
                let prompts = serde_json::from_str::<Vec<Prompt>>(&value)
                    .ok()
                    .unwrap_or(vec![]);

                set_prompts(cx, prompts);
            }
            _ => {}
        });

        let prompts = storage
            .get(StorageKey::Prompts)
            .and_then(|value| serde_json::from_str::<Vec<Prompt>>(&value).ok())
            .unwrap_or(vec![]);

        let state: Entity<AppState> = cx.new(|_cx| AppState {
            active_prompt_id: None,
            active_view: AppView::AssistantView,
            blur_id: 0,
            error: None,
            input: None,
            output: "".to_owned(),
            prompts,

            focused: false,
            loading: false,
            visible: true,
            assistant_processing: false,
            settings_window_handle: None,
        });

        let app_state = AppStateController { state };

        cx.set_global(app_state);
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

    pub fn set_active_prompt_id(&self, cx: &mut App, id: Option<String>) {
        self.state.update(cx, |state, cx| {
            state.active_prompt_id = id.clone();

            if let Some(id) = id {
                cx.emit(AppEvent::AssistantChanged(id));
            }

            cx.notify();
        });
    }

    pub fn set_promts(&self, cx: &mut App, prompts: Vec<Prompt>) {
        self.state.update(cx, |state, cx| {
            state.prompts = prompts;
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

    pub fn set_assistant_processing(&self, cx: &mut App, processing: bool) {
        self.state.update(cx, |state, cx| {
            state.assistant_processing = processing;
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
        // Should not hide app if settings window is open.
        if self.state.read(cx).settings_window_handle.is_some() {
            return;
        }

        self.state.update(cx, |model, cx| {
            model.visible = visible;
            cx.notify();

            if visible {
                cx.activate(true);
            } else {
                cx.hide();
            }
        });
    }

    pub fn set_settings_window_handle(&self, cx: &mut App, handle: Option<WindowHandle<Root>>) {
        self.state.update(cx, |state, cx| {
            state.settings_window_handle = handle;
            cx.notify();
        });
    }
}

/* Helper functions */

pub fn get_active_prompt(cx: &App) -> Option<Prompt> {
    let state = cx.global::<AppStateController>().state.read(cx);

    match state.active_prompt_id.clone() {
        Some(id) => state.prompts.iter().find(|prompt| prompt.id == id).cloned(),
        None => None,
    }
}

pub fn get_asssistant_processing(cx: &App) -> bool {
    let state = cx.global::<AppStateController>().state.read(cx);
    state.assistant_processing
}

pub fn set_active_prompt_id(cx: &mut App, id: Option<String>) {
    AppStateController::update(|this, cx| this.set_active_prompt_id(cx, id), cx);
}

pub fn set_active_view(cx: &mut App, view: AppView) {
    AppStateController::update(|this, cx| this.set_active_view(cx, view), cx);
}

pub fn set_assistant_processing(cx: &mut App, processing: bool) {
    AppStateController::update(|this, cx| this.set_assistant_processing(cx, processing), cx);
}

pub fn set_assistant_processing_async(cx: &mut AsyncApp, processing: bool) {
    AppStateController::update_async(|this, cx| this.set_assistant_processing(cx, processing), cx);
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

pub fn set_prompts(cx: &mut App, prompts: Vec<Prompt>) {
    AppStateController::update(|this, cx| this.set_promts(cx, prompts), cx);
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

pub fn get_settings_window_handle(cx: &App) -> Option<WindowHandle<Root>> {
    let state = cx.global::<AppStateController>().state.read(cx);
    state.settings_window_handle
}

pub fn set_settings_window_handle(cx: &mut App, handle: Option<WindowHandle<Root>>) {
    AppStateController::update(|this, cx| this.set_settings_window_handle(cx, handle), cx);
}
