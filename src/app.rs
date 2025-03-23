use async_std::stream::StreamExt;
use gpui::{
    actions, div, prelude::*, App, AppContext, Entity, EventEmitter, FocusHandle, KeyBinding,
    Window,
};

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::services::{Api, Auth, Storage};
use crate::state::*;
use crate::theme::Theme;
use crate::ui::*;
use crate::utils;

actions!(app, [OpenSettings]);

pub struct AppRoot {
    assistant_view: Entity<AssistantView>,
    error_view: Entity<ErrorView>,
    login_view: Entity<LoginView>,
    profile_view: Entity<ProfileView>,

    back_button: Entity<BackButton>,
    profile_button: Entity<ProfileButton>,

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

        let auth = cx.global::<Auth>().clone();
        let global_state = cx.global::<GlobalState>().clone();

        // Try to authenticate user if token is present
        // If token is absent, nothing to do, need to login first
        // If token is expired, try to refresh it and retry to authenticate
        cx.spawn(|mut cx| async move {
            let user = auth.get_user(&mut cx).await;

            match user {
                Ok(user) => {
                    set_user_async(&mut cx, Some(user));
                    set_authenticated_async(&mut cx, true);
                }
                Err(err) => match err
                    .downcast_ref::<AuthError>()
                    .unwrap_or(&AuthError::UnknownError)
                {
                    AuthError::InvalidTokenError(_) => {
                        let user = auth.refresh_access_token(&mut cx).await;

                        match user {
                            Ok(user) => {
                                set_user_async(&mut cx, Some(user));
                                set_authenticated_async(&mut cx, true);
                            }
                            Err(_err) => {
                                // refresh token is probably expired, need to login again
                            }
                        }
                    }
                    AuthError::NoTokenError => {
                        // nothing to do, need to login first
                    }
                    _ => set_error_async(&mut cx, Some(err)),
                },
            };
        })
        .detach();

        let _app_events_subscribtion = cx
            .subscribe(&global_state.state, |state, event, cx| {
                let _ = match event.clone() {
                    AppEvent::Authenticated => {
                        let api = cx.global::<Api>().clone();
                        let storage = cx.global::<Storage>().clone();

                        cx.spawn(|mut cx| async move {
                            let assistants = api.get_assistants(&mut cx).await;
                            let saved_assistant_id = storage.get("assistant_id".into());

                            GlobalState::update_async(
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
                                        this.set_error(cx, Some(err));
                                    }
                                },
                                &mut cx,
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

                        cx.spawn(|mut cx| async move {
                            let output = assistant.generate_response(input).await;

                            set_loading_async(&mut cx, false);

                            let _ = match output {
                                Ok(mut stream) => {
                                    while let Some(item) = stream.next().await {
                                        append_output_async(&mut cx, item);
                                    }
                                }
                                Err(err) => set_error_async(&mut cx, Some(err)),
                            };
                        })
                        .detach();
                    }
                    AppEvent::VisibilityChanged(visible) => {
                        let stack = cx.windows();

                        if let Some(window_handle) = stack.get(0) {
                            let _ = window_handle.update(cx, |_view, window, cx| {
                                let height = state.read(cx).content_height;
                                window.set_frame(utils::app_window_bounds(cx, height, visible));
                            });
                        }
                    }
                    _ => {}
                };
            })
            .detach();

        let view = cx.new(move |cx| {
            let state = global_state.state.clone();
            let assistant_view = cx.new(|cx| AssistantView::new(cx, &state));
            let error_view = cx.new(|cx| ErrorView::new(cx, &state));
            let login_view = cx.new(|cx| LoginView::new(cx, &state));
            let profile_view = cx.new(|cx| ProfileView::new(cx, &state));

            let back_button = cx.new(|cx| BackButton::new(cx, &state));
            let profile_button = cx.new(|cx| ProfileButton::new(cx, &state));

            let _ = cx
                .observe(&state, |this: &mut AppRoot, state, cx| {
                    this.visible = state.read(cx).visible;
                    cx.notify();
                })
                .detach();

            AppRoot {
                assistant_view,
                error_view,
                login_view,
                profile_view,

                back_button,
                profile_button,

                visible: state.read(cx).visible,
                focus_handle,
            }
        });

        view
    }

    fn open_settings(&mut self, _: &OpenSettings, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(AppEvent::OpenSettings);
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let _title_row = div().flex().flex_row().items_start().p_2();
        let content = div().flex().flex_col().flex_grow().pt_4().pb_3().px_3();

        let back_button = div().flex().mr_2().child(self.back_button.clone());
        let profile_button = div().flex().mr_1().child(self.profile_button.clone());

        let mut corner_buttons = vec![];
        // only one button is visible per time
        corner_buttons.push(back_button);
        corner_buttons.push(profile_button);

        let assistant_view = div().child(self.assistant_view.clone());
        let login_view = div().child(self.login_view.clone());
        let profile_view = div().child(self.profile_view.clone());
        let visible = self.visible.clone();

        let content = div()
            .on_children_prepainted(move |bounds, window, cx| {
                let content_height: f32 = bounds.iter().map(|b| b.size.height.0).sum();
                set_content_height(cx, content_height);
                window.set_frame(utils::app_window_bounds(cx, content_height, visible));
            })
            //.child(title_row.children(corner_buttons))
            .child(content.children([assistant_view, login_view, profile_view])) // only one view is visible per time
            .child(self.error_view.clone());

        div()
            .on_action(cx.listener(Self::open_settings))
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
