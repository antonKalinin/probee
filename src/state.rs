use gpui::*;

use crate::services::*;
use crate::window::Window;

#[derive(Debug)]
pub struct State {
    pub mode: AssistMode,
    pub input: Option<String>,
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
                mode: AssistMode::Translate,
                input: None,
                output: "...".to_string(),
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
            Window::set_height(wcx, size.height.0 + 40.);
        }
    }

    pub fn set_mode(&self, wcx: &mut WindowContext, mode: AssistMode) {
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

            let mode = state.mode.clone();

            cx.spawn(|mut cx| async move {
                let output = assistant.ask(mode, &input).await;

                if let Ok(text) = output {
                    Self::update_async(
                        |this, cx| {
                            this.set_output(cx, text);
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
