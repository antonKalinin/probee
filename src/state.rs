use anyhow::Error;
use gpui::*;

use crate::services::*;
use crate::window::Window;

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    AppView,
    AssitantView,
}

#[derive(Debug)]
pub struct State {
    pub active_view: ActiveView,
    pub error: Option<Error>,
    pub input: Option<String>,
    pub loading: bool,
    pub mode: Option<AssistMode>,
    pub output: String,
    pub output_size: Option<Size<Pixels>>,
}

#[derive(Clone)]
pub struct StateController {
    pub model: Model<State>,
}

impl StateController {
    pub fn init(cx: &mut WindowContext) -> Self {
        let this = Self {
            model: cx.new_model(|_| State {
                active_view: ActiveView::AppView,
                error: None,
                input: None,
                loading: false,
                mode: Some(AssistMode::Translate),
                output: "".to_string(),
                output_size: None,
            }),
        };

        cx.set_global(this.clone());

        this
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

    pub fn set_active_view(&self, wcx: &mut WindowContext, view: ActiveView) {
        self.model.update(wcx, |model, cx| {
            model.active_view = view;
            cx.notify();
        });
    }

    pub fn set_input(&self, wcx: &mut WindowContext, input: String) {
        self.model.update(wcx, |model, cx| {
            model.input = Some(input);
            cx.notify();
        });
    }

    pub fn set_output(&self, wcx: &mut WindowContext, output: String) {
        self.model.update(wcx, |model, cx| {
            model.output = output;
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

    pub fn set_output_size(&self, wcx: &mut WindowContext, size: Size<Pixels>) {
        let mut resized = false;

        self.model.update(wcx, |model, _cx| {
            if let Some(prev_size) = model.output_size {
                resized = prev_size != size;
            } else {
                resized = true;
            }

            if resized {
                model.output_size = Some(size);
            }
        });

        if resized {
            Window::set_height(wcx, size.height.0 + 80.);
        }
    }

    pub fn set_mode(&self, wcx: &mut WindowContext, mode: Option<AssistMode>) {
        self.model.update(wcx, |model, cx| {
            model.mode = mode;
            cx.notify();
        });
    }

    pub fn request_assistant(&self, cx: &mut WindowContext) {
        let state = self.model.read(cx);
        let assistant = cx.global::<Assistant>().clone();

        if let Some(input) = state.input.clone() {
            if input.is_empty() {
                return;
            }

            let mode = state.mode.clone().unwrap();

            cx.spawn(|mut cx| async move {
                let output = assistant.ask(mode, &input).await;

                if let Ok(text) = output {
                    Self::update_async(
                        |this, cx| {
                            this.set_output(cx, text);
                            this.set_loading(cx, false);
                        },
                        &mut cx,
                    );
                }
            })
            .detach();
        }
    }
}

impl Global for StateController {}
