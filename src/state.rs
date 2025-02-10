use anyhow::Error;
use gpui::{App, AppContext, AsyncApp, BorrowAppContext, Entity, EventEmitter, Global};

use crate::events::AppEvent;
use crate::services::{AssistantConfig, Auth, User};

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    AssitantView,
    LoginView,
    ProfileView,
}

#[derive(Debug)]
pub struct State {
    pub active_assistant_id: Option<String>,
    pub active_view: ActiveView,
    pub assistants: Vec<AssistantConfig>,
    pub authenticated: bool,
    pub error: Option<Error>,
    pub input: Option<String>,
    pub loading: bool,
    pub output: String,
    pub user: Option<User>,
}

impl EventEmitter<AppEvent> for State {}

#[derive(Clone)]
pub struct GlobalState {
    pub state: Entity<State>,
}

impl Global for GlobalState {}

impl GlobalState {
    pub fn init(cx: &mut App) {
        let state: Entity<State> = cx.new(|_cx| State {
            active_assistant_id: None,
            active_view: ActiveView::AssitantView,
            assistants: vec![],
            authenticated: false,
            error: None,
            input: None,
            loading: false,
            output: "".to_owned(),
            user: None,
        });

        let global_state = GlobalState { state };

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

    pub fn set_active_view(&self, cx: &mut App, view: ActiveView) {
        self.state.update(cx, |state, cx| {
            state.active_view = view.clone();
            state.error = None;

            cx.notify();
        });
    }

    pub fn set_authenticated(&self, cx: &mut App, authenticated: bool) {
        self.state.update(cx, |model, cx| {
            model.authenticated = authenticated;
            cx.notify();
            cx.emit(AppEvent::Authenticated);
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

    pub fn set_error(&self, cx: &mut App, error: Option<Error>) {
        self.state.update(cx, |state, cx| {
            state.error = error;
            cx.notify();
        });
    }

    pub fn set_loading(&self, cx: &mut App, loading: bool) {
        self.state.update(cx, |model, cx| {
            model.loading = loading;
            cx.notify();
        });
    }

    pub fn set_user(&self, cx: &mut App, user: Option<User>) {
        self.state.update(cx, |model, cx| {
            model.user = user;
            cx.notify();
        });
    }
}

/* Helper functions */

pub fn get_active_assistant(cx: &App) -> Option<AssistantConfig> {
    let state = cx.global::<GlobalState>().state.read(cx);

    match state.active_assistant_id.clone() {
        Some(id) => state
            .assistants
            .iter()
            .find(|assistant| assistant.id == id)
            .cloned(),
        None => None,
    }
}

pub fn set_active_assistant_id(cx: &mut App, id: Option<String>) {
    GlobalState::update(|this, cx| this.set_active_assistant_id(cx, id), cx);
}

pub fn set_active_view(cx: &mut App, view: ActiveView) {
    GlobalState::update(|this, cx| this.set_active_view(cx, view), cx);
}

pub fn set_active_view_async(cx: &mut AsyncApp, view: ActiveView) {
    GlobalState::update_async(|this, cx| this.set_active_view(cx, view), cx);
}

pub fn set_input(cx: &mut App, input: String) {
    GlobalState::update(|this, cx| this.set_input(cx, input), cx);
}

pub fn set_output(cx: &mut App, output: String) {
    GlobalState::update(|this, cx| this.set_output(cx, output), cx);
}

pub fn append_output_async(cx: &mut AsyncApp, output: String) {
    GlobalState::update_async(|this, cx| this.append_output(cx, output), cx);
}

pub fn set_loading(cx: &mut App, loading: bool) {
    GlobalState::update(|this, cx| this.set_loading(cx, loading), cx);
}

pub fn set_loading_async(cx: &mut AsyncApp, loading: bool) {
    GlobalState::update_async(|this, cx| this.set_loading(cx, loading), cx);
}

pub fn set_error(cx: &mut App, error: Option<Error>) {
    GlobalState::update(|this, cx| this.set_error(cx, error), cx);
}

pub fn set_error_async(cx: &mut AsyncApp, error: Option<Error>) {
    GlobalState::update_async(|this, cx| this.set_error(cx, error), cx);
}

pub fn set_authenticated_async(cx: &mut AsyncApp, authenticated: bool) {
    GlobalState::update_async(|this, cx| this.set_authenticated(cx, authenticated), cx);
}

pub fn set_user_async(cx: &mut AsyncApp, user: Option<User>) {
    GlobalState::update_async(|this, cx| this.set_user(cx, user), cx);
}
