use async_std::stream::StreamExt;
use gpui::{
    actions, div, prelude::*, App, AppContext, Entity, EventEmitter, FocusHandle, KeyBinding,
    Window,
};

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::services::{Api, Auth, Storage};
use crate::state::app::*;
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
    settings_opened: bool,
    focus_handle: FocusHandle,
}

impl AppRoot {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        // Focus handle is to track when focus received by the window
        // so in-app shortsucts can be listened
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        cx.bind_keys([KeyBinding::new("cmd-,", OpenSettings, None)]);
        cx.bind_keys([KeyBinding::new("alt-1", SelectPrevAssistant, None)]);
        cx.bind_keys([KeyBinding::new("alt-2", SelectNextAssistant, None)]);
        cx.bind_keys([KeyBinding::new("alt-`", ToggleLibraryView, None)]);

        let auth = cx.global::<Auth>().clone();
        let global_state = cx.global::<AppStateController>().clone();

        // Try to authenticate user if token is present
        // If token is absent, nothing to do, need to login first
        // If token is expired, try to refresh it and retry to authenticate
        cx.spawn(async move |cx| {
            let user = auth.get_user(cx).await;

            // match user {
            //     Ok(user) => {
            //         set_user_async(cx, Some(user));
            //         set_authenticated_async(cx, true);
            //     }
            //     Err(err) => match err
            //         .downcast_ref::<AuthError>()
            //         .unwrap_or(&AuthError::UnknownError)
            //     {
            //         AuthError::InvalidTokenError(_) => {
            //             let user = auth.refresh_access_token(cx).await;

            //             match user {
            //                 Ok(user) => {
            //                     set_user_async(cx, Some(user));
            //                     set_authenticated_async(cx, true);
            //                 }
            //                 Err(_err) => {
            //                     // refresh token is probably expired, need to login again
            //                 }
            //             }
            //         }
            //         AuthError::NoTokenError => {
            //             // nothing to do, need to login first
            //         }
            //         _ => set_error_async(cx, Some(err)),
            //     },
            // };
        })
        .detach();

        let _app_events_subscribtion = cx
            .subscribe(&global_state.state, |_state, event, cx| {
                let _ = match event.clone() {
                    AppEvent::Authenticated => {
                        let api = cx.global::<Api>().clone();
                        let storage = cx.global::<Storage>().clone();

                        cx.spawn(async move |cx| {
                            let assistants = api.get_assistants(cx).await;
                            let saved_assistant_id = storage.get("assistant_id".into());

                            AppStateController::update_async(
                                |this, cx| match assistants {
                                    Ok(assistants) => {
                                        this.set_assistants(cx, assistants.clone());
                                        let first_assistant_id =
                                            assistants.first().map(|a| a.id.clone());

                                        match (saved_assistant_id, first_assistant_id) {
                                            (Some(id), _) | (None, Some(id)) => {
                                                this.set_active_assistant_id(cx, Some(id))
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
                    }
                    AppEvent::AssistantChanged(id) => {
                        let storage = cx.global_mut::<Storage>();
                        let _ = storage.set("assistant_id".into(), id.clone());
                        // TODO: As soon as assistant is changed, reset it in cx.global
                    }
                    AppEvent::InputChanged(input) => {
                        let mut assistant = cx.global::<Assistant>().clone();
                        let assistant_config = get_active_assistant(cx);
                        if assistant_config.is_none() {
                            let err = AssistantError::MissingConfig.into();
                            set_error(cx, Some(err));
                            return;
                        }

                        // TODO: Config should not be reset on every input change
                        let _ = assistant.set_config(assistant_config.unwrap().clone());

                        set_error(cx, None);
                        set_output(cx, "".to_owned());
                        set_loading(cx, true);

                        cx.spawn(async move |cx| {
                            let output = assistant.generate_response(input).await;

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
            let state = global_state.state.clone();
            let assistant_view = cx.new(|cx| AssistantView::new(cx, &state));
            let library_view = cx.new(|cx| LibraryView::new(cx, &state));
            let error_view = cx.new(|cx| ErrorView::new(cx, &state));

            let _ = cx
                .observe(&state, |this: &mut AppRoot, state, cx| {
                    this.visible = state.read(cx).visible;
                    cx.notify();
                })
                .detach();

            AppRoot {
                assistant_view,
                library_view,
                error_view,

                visible: state.read(cx).visible,
                settings_opened: false,
                focus_handle,
            }
        });

        view
    }

    fn open_settings(&mut self, _: &OpenSettings, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(AppEvent::OpenSettings);
    }

    fn select_next_assistant(
        &mut self,
        _: &SelectNextAssistant,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = cx.global::<AppStateController>().clone().state.read(cx);
        let assistants = app_state.assistants.clone();
        let curr_assistant_id = app_state.active_assistant_id.clone();

        if curr_assistant_id.is_none() {
            return;
        }

        let curr_assistant_index = (assistants
            .iter()
            .position(|assistant| &assistant.id == curr_assistant_id.as_ref().unwrap()))
        .unwrap_or(0);

        let next_assistant_index = if curr_assistant_index == assistants.len() - 1 {
            0
        } else {
            curr_assistant_index + 1
        };

        if let Some(assistant) = assistants.get(next_assistant_index) {
            set_active_assistant_id(cx, Some(assistant.id.clone()));
        }
    }

    fn select_prev_assistant(
        &mut self,
        _: &SelectPrevAssistant,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = cx.global::<AppStateController>().clone().state.read(cx);
        let assistants = app_state.assistants.clone();
        let curr_assistant_id = app_state.active_assistant_id.clone();

        if curr_assistant_id.is_none() {
            return;
        }

        let curr_assistant_index = (assistants
            .iter()
            .position(|assistant| &assistant.id == curr_assistant_id.as_ref().unwrap()))
        .unwrap_or(0);

        let next_assistant_index = if curr_assistant_index == 0 {
            assistants.len() - 1
        } else {
            curr_assistant_index - 1
        };

        if let Some(assistant) = assistants.get(next_assistant_index) {
            set_active_assistant_id(cx, Some(assistant.id.clone()));
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
                set_content_height(cx, content_height);
                window.set_frame(utils::app_window_bounds(cx, content_height));
            })
            .child(content.children([assistant_view, library_view])) // only one view is visible per time
            .child(self.error_view.clone());

        div()
            .on_action(cx.listener(Self::open_settings))
            .on_action(cx.listener(Self::select_next_assistant))
            .on_action(cx.listener(Self::select_prev_assistant))
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
