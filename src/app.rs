use async_std::stream::StreamExt;
use gpui::{
    div, prelude::*, App, AppContext, Entity, EventEmitter, FocusHandle, KeyBinding, Window,
};

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::services::{selection, Api, Storage, StorageKey};
use crate::state::app_state::*;
use crate::ui::*;
use crate::utils;
use crate::utils::actions::{
    OpenSettings, RunAssistant, SelectNextAssistant, SelectPrevAssistant, ToogleVisibility,
};

pub struct AppRoot {
    assistant_view: Entity<AssistantView>,
    library_view: Entity<LibraryView>,
    error_view: Entity<ErrorView>,

    visible: bool,
    focus_handle: FocusHandle,
}

impl AppRoot {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        // Focus handle is to track when focus received by the window
        // so in-app shortsucts can be listened
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let api = cx.global::<Api>().clone();
        let models = Model::get_models();
        let default_model = models.get(0).unwrap();
        let storage = cx.global::<Storage>().clone();
        let state_controller = cx.global::<AppStateController>().clone();

        // Set default values to storage
        let _ = storage.set_default(StorageKey::HotkeyPrevPropmt, "alt+1".to_string());
        let _ = storage.set_default(StorageKey::HotkeyNextPrompt, "alt+2".to_string());
        let _ = storage.set_default(StorageKey::HotkeyRunAssistant, "alt+alt".to_string());
        let _ = storage.set_default(StorageKey::HotkeyToogleVisibility, "alt+tab".to_string());
        let _ = storage.set_default(
            StorageKey::AssistantModel,
            serde_json::to_string(default_model).unwrap(),
        );

        let prev_prompt_hk = storage.get(StorageKey::HotkeyPrevPropmt).unwrap();
        let next_prompt_hk = storage.get(StorageKey::HotkeyNextPrompt).unwrap();

        cx.bind_keys([KeyBinding::new("cmd-,", OpenSettings, None)]);
        cx.bind_keys([KeyBinding::new(&prev_prompt_hk, SelectPrevAssistant, None)]);
        cx.bind_keys([KeyBinding::new(&next_prompt_hk, SelectNextAssistant, None)]);

        // Global actions bindings
        cx.on_action(|_: &ToogleVisibility, cx| {
            toggle_visible(cx);
        });

        cx.on_action(|_: &RunAssistant, cx| {
            let input_text = selection::get_text();

            match input_text {
                Ok(text) => {
                    if text.is_empty() {
                        set_error(cx, Some(InputError::EmptyTextInputError.into()));
                    } else {
                        set_input(cx, text);
                    }
                }
                Err(err) => {
                    set_error(cx, Some(err));
                }
            }

            set_visible(cx, true);
        });

        // Load prompts from API to store them and set to state
        cx.spawn(async move |cx| {
            let app_prompts = api.get_prompts(cx).await;
            let saved_propmt_id = storage.get(StorageKey::AssistantId);

            match app_prompts {
                Ok(app_prompts) => {
                    let app_prompts = app_prompts.into_iter().take(3).collect::<Vec<_>>();
                    let app_prompt_ids =
                        app_prompts.iter().map(|a| a.id.clone()).collect::<Vec<_>>();

                    let mut prompts = storage
                        .get(StorageKey::Prompts)
                        .and_then(|value| serde_json::from_str::<Vec<Prompt>>(&value).ok())
                        .unwrap_or(vec![])
                        .into_iter()
                        .filter(|p| !app_prompt_ids.contains(&p.id))
                        .collect::<Vec<_>>();

                    prompts.extend(app_prompts.iter().cloned());

                    let _ = storage.set_notify_async(
                        StorageKey::Prompts,
                        serde_json::to_string(&prompts).unwrap(),
                        cx,
                    );

                    let _ = storage.set(
                        StorageKey::AppPropmptIds,
                        serde_json::to_string(&app_prompt_ids).unwrap(),
                    );

                    AppStateController::update_async(
                        |this, cx| {
                            let prompts_ids =
                                prompts.iter().map(|a| a.id.clone()).collect::<Vec<_>>();
                            let first_prompt_id = prompts_ids.first().cloned();

                            // ensure if the saved prompt id is still valid
                            let saved_propmt_id = saved_propmt_id
                                .as_ref()
                                .filter(|id| prompts_ids.contains(id))
                                .cloned();

                            match (saved_propmt_id, first_prompt_id) {
                                (Some(id), _) | (None, Some(id)) => {
                                    this.set_active_prompt_id(cx, Some(id))
                                }
                                _ => {}
                            }
                        },
                        cx,
                    );
                }
                Err(err) => {
                    set_error_async(cx, Some(err));
                }
            }
        })
        .detach();

        let _app_events_subscribtion = cx
            .subscribe(&state_controller.state, |_state, event, cx| {
                let _ = match event.clone() {
                    AppEvent::AssistantChanged(id) => {
                        let storage = cx.global_mut::<Storage>();
                        let _ = storage.set(StorageKey::AssistantId, id.clone());
                        // TODO: As soon as prompt is changed, reset it in cx.global
                    }
                    AppEvent::InputChanged(input) => {
                        let mut assitant = cx.global::<Assistant>().clone();
                        let prompt = get_active_prompt(cx);

                        if prompt.is_none() {
                            let err = AssistantError::MissingPrompt.into();
                            set_error(cx, Some(err));
                            return;
                        }

                        // TODO: Config should not be reset on every input change
                        let _ = assitant.set_prompt(prompt.unwrap());

                        set_error(cx, None);
                        set_output(cx, "".to_owned());
                        set_loading(cx, true);

                        cx.spawn(async move |cx| {
                            let output = assitant.generate_response(input).await;

                            set_loading_async(cx, false);

                            let _ = match output {
                                Ok(mut stream) => {
                                    while let Some(item) = stream.next().await {
                                        append_output_async(cx, item);
                                    }
                                }
                                Err(err) => set_error_async(cx, Some(err)),
                            };
                        })
                        .detach();
                    }
                    _ => {}
                };
            })
            .detach();

        let view = cx.new(move |cx| {
            let state = state_controller.state.clone();
            let assistant_view = cx.new(|cx| AssistantView::new(cx, &state));
            let library_view = cx.new(|cx| LibraryView::new(cx, &state));
            let error_view = cx.new(|cx| ErrorView::new(cx, &state));

            cx.observe(&state, |this: &mut AppRoot, state, cx| {
                this.visible = state.read(cx).visible;
                cx.notify();
            })
            .detach();

            cx.on_focus(&focus_handle, window, |_this, _window, cx| {
                set_focused(cx, true);
            })
            .detach();

            cx.on_blur(&focus_handle, window, |_this, _window, cx| {
                set_focused(cx, false);
                set_visible(cx, false);
            })
            .detach();

            AppRoot {
                assistant_view,
                library_view,
                error_view,

                visible: state.read(cx).visible,
                focus_handle,
            }
        });

        view
    }

    fn open_settings(&mut self, _: &OpenSettings, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(AppEvent::OpenSettings);
    }

    fn select_next_prompt(
        &mut self,
        _: &SelectNextAssistant,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = cx.global::<AppStateController>().clone().state.read(cx);
        let prompts = app_state.prompts.clone();
        let curr_prompt_id = app_state.active_prompt_id.clone();

        if curr_prompt_id.is_none() {
            return;
        }

        let curr_prompt_index = (prompts
            .iter()
            .position(|prompt| &prompt.id == curr_prompt_id.as_ref().unwrap()))
        .unwrap_or(0);

        let next_prompt_index = if curr_prompt_index == prompts.len() - 1 {
            0
        } else {
            curr_prompt_index + 1
        };

        if let Some(prompt) = prompts.get(next_prompt_index) {
            set_active_prompt_id(cx, Some(prompt.id.clone()));
            set_output(cx, "".to_owned());
        }
    }

    fn select_prev_prompt(
        &mut self,
        _: &SelectPrevAssistant,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = cx.global::<AppStateController>().clone().state.read(cx);
        let prompts = app_state.prompts.clone();
        let curr_prompt_id = app_state.active_prompt_id.clone();

        if curr_prompt_id.is_none() {
            return;
        }

        let curr_prompt_index = (prompts
            .iter()
            .position(|prompt| &prompt.id == curr_prompt_id.as_ref().unwrap()))
        .unwrap_or(0);

        let next_prompt_index = if curr_prompt_index == 0 {
            prompts.len() - 1
        } else {
            curr_prompt_index - 1
        };

        if let Some(prompt) = prompts.get(next_prompt_index) {
            set_active_prompt_id(cx, Some(prompt.id.clone()));
            set_output(cx, "".to_owned());
        }
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let content = div().flex().flex_col().flex_grow().pt_4().pb_3().px_3();

        let assistant_view = div().child(self.assistant_view.clone());
        let library_view = div().child(self.library_view.clone());

        let content = div()
            .on_children_prepainted(move |bounds, window, cx| {
                let content_height: f32 = bounds.iter().map(|b| b.size.height.0).sum();
                window.set_frame(utils::app_window_bounds(cx, content_height), true);
            })
            .child(content.children([assistant_view, library_view])) // only one view is visible per time
            .child(self.error_view.clone());

        div()
            .on_action(cx.listener(Self::open_settings))
            .on_action(cx.listener(Self::select_next_prompt))
            .on_action(cx.listener(Self::select_prev_prompt))
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .border_color(theme.border)
            .child(content)
    }
}

impl EventEmitter<AppEvent> for AppRoot {}
