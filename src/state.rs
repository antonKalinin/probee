use anyhow::Error;
use gpui::*;

use crate::events::*;
use crate::services::*;
use crate::window::Window;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    AppView,
    AssitantView,
}

#[derive(Debug)]
pub struct State {
    pub active_assistant_id: Option<String>,
    pub active_view: ActiveView,
    pub assistants: Vec<AssistantConfig>,
    pub content_size: Option<Size<Pixels>>,
    pub error: Option<Error>,
    pub input: Option<String>,
    pub loading: bool,
    pub output: String,
}

impl EventEmitter<AppEvent> for State {}

#[derive(Clone)]
pub struct StateController {
    pub model: Model<State>,
}

impl Global for StateController {}

impl StateController {
    pub fn init(cx: &mut AppContext) {
        let this = Self {
            model: cx.new_model(|_| State {
                active_assistant_id: None,
                active_view: ActiveView::AppView,
                assistants: vec![],
                content_size: None,
                error: None,
                input: None,
                loading: false,
                output: "".to_owned(),
            }),
        };

        cx.set_global(this.clone());
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        if !cx.has_global::<Self>() {
            return;
        }
        cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        });
    }

    pub fn update_async(
        f: impl FnOnce(&mut Self, &mut WindowContext),
        cx: &mut AsyncWindowContext,
    ) {
        let _ = cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        });
    }

    pub fn set_active_assistant_id(&self, wcx: &mut WindowContext, id: Option<String>) {
        self.model.update(wcx, |model, cx| {
            model.active_assistant_id = id.clone();

            if let Some(id) = id {
                cx.emit(AppEvent::AssistantChanged(id));
            }

            cx.notify();
        });
    }

    pub fn set_assistants(&self, wcx: &mut WindowContext, assistants: Vec<AssistantConfig>) {
        self.model.update(wcx, |model, cx| {
            model.assistants = assistants;
            cx.notify();
        });
    }

    pub fn set_active_view(&self, wcx: &mut WindowContext, view: ActiveView) {
        self.model.update(wcx, |model, cx| {
            model.active_view = view;
            cx.notify();
        });
    }

    pub fn set_input(&self, wcx: &mut WindowContext, input: String) {
        self.model.update(wcx, |model, cx| {
            model.input = Some(input.clone());
            cx.notify();
            cx.emit(AppEvent::InputChanged(input));
        });
    }

    pub fn set_output(&self, wcx: &mut WindowContext, output: String) {
        self.model.update(wcx, |model, cx| {
            model.output = output;
            cx.notify();
        });
    }

    pub fn append_output(&self, wcx: &mut WindowContext, output: String) {
        self.model.update(wcx, |model, cx| {
            model.output.push_str(&output);
            cx.notify();
        });
    }

    pub fn set_error(&self, wcx: &mut WindowContext, error: Option<Error>) {
        self.model.update(wcx, |model, cx| {
            model.error = error;
            cx.notify();
        });
    }

    pub fn set_loading(&self, wcx: &mut WindowContext, loading: bool) {
        self.model.update(wcx, |model, cx| {
            model.loading = loading;
            cx.notify();
        });
    }

    pub fn set_content_size(&self, wcx: &mut WindowContext, size: Size<Pixels>) {
        let mut resized = false;

        self.model.update(wcx, |model, _cx| {
            if let Some(prev_size) = model.content_size {
                resized = prev_size != size;
            } else {
                resized = true;
            }

            if resized {
                model.content_size = Some(size);
            }
        });

        if resized {
            Window::set_height(wcx, size.height.0);
        }
    }
}

/* Helper functions */

pub fn get_active_assistant(cx: &WindowContext) -> Option<AssistantConfig> {
    let state = cx.global::<StateController>().model.read(cx);

    match state.active_assistant_id.clone() {
        Some(id) => state
            .assistants
            .iter()
            .find(|assistant| assistant.id == id)
            .cloned(),
        None => None,
    }
}

pub fn set_active_assistant_id(cx: &mut WindowContext, id: Option<String>) {
    StateController::update(|this, cx| this.set_active_assistant_id(cx, id), cx);
}

pub fn set_active_view(cx: &mut WindowContext, view: ActiveView) {
    StateController::update(|this, cx| this.set_active_view(cx, view), cx);
}

pub fn set_input(cx: &mut WindowContext, input: String) {
    StateController::update(|this, cx| this.set_input(cx, input), cx);
}

pub fn set_output(cx: &mut WindowContext, output: String) {
    StateController::update(|this, cx| this.set_output(cx, output), cx);
}

pub fn append_output_async(cx: &mut AsyncWindowContext, output: String) {
    StateController::update_async(|this, cx| this.append_output(cx, output), cx);
}

pub fn set_loading(cx: &mut WindowContext, loading: bool) {
    StateController::update(|this, cx| this.set_loading(cx, loading), cx);
}

pub fn set_loading_async(cx: &mut AsyncWindowContext, loading: bool) {
    StateController::update_async(|this, cx| this.set_loading(cx, loading), cx);
}

pub fn set_error(cx: &mut WindowContext, error: Option<Error>) {
    StateController::update(|this, cx| this.set_error(cx, error), cx);
}

pub fn set_error_async(cx: &mut AsyncWindowContext, error: Option<Error>) {
    StateController::update_async(|this, cx| this.set_error(cx, error), cx);
}
