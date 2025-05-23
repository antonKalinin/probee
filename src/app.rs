use async_std::stream::StreamExt;
use gpui::{
    actions, div, prelude::*, App, AppContext, Entity, EventEmitter, FocusHandle, KeyBinding,
    Window,
};
use std::time::Duration;

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::services::{Api, Storage, StorageKey};
use crate::state::app_state::*;
use crate::ui::*;
use crate::utils;

actions!(
    app,
    [
        OpenSettings,
        SelectNextAssistant,
        SelectPrevAssistant,
        ToggleLibraryView
    ]
);

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

        cx.bind_keys([KeyBinding::new("cmd-,", OpenSettings, None)]);
        // TODO: Load this bindings from storage
        cx.bind_keys([KeyBinding::new("alt-1", SelectPrevAssistant, None)]);
        cx.bind_keys([KeyBinding::new("alt-2", SelectNextAssistant, None)]);
        cx.bind_keys([KeyBinding::new("alt-`", ToggleLibraryView, None)]);

        let api = cx.global::<Api>().clone();
        let storage = cx.global::<Storage>().clone();
        let state_controller = cx.global::<AppStateController>().clone();

        cx.spawn(async move |cx| {
            let prompts = api.get_prompts(cx).await;
            let saved_propmt_id = storage.get(StorageKey::AssistantId);

            AppStateController::update_async(
                |this, cx| match prompts {
                    Ok(prompts) => {
                        this.set_promts(cx, prompts.clone());
                        let prompts_ids = prompts.iter().map(|a| a.id.clone()).collect::<Vec<_>>();
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
                    }
                    Err(err) => {
                        set_error(cx, Some(err));
                    }
                },
                cx,
            );
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
                let blur_id = get_blur_id(cx);

                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_secs(5)).await;

                    this.update(cx, |_this, cx| {
                        // hide the the only window if it is not focused more than 5 seconds
                        if !get_focused(cx) && get_blur_id(cx) == blur_id && cx.windows().len() == 1
                        {
                            set_visible(cx, false);
                        }
                    })
                    .ok();
                })
                .detach();
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
