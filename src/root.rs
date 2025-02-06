use async_std::stream::StreamExt;
use gpui::{div, prelude::*, App, Div, Entity, EventEmitter, Render, Window};

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::services::Auth;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::*;
use crate::utils;

pub struct Root {
    assistant_view: Entity<AssistantView>,
    error_view: Entity<ErrorView>,
    login_view: Entity<LoginView>,
    profile_view: Entity<ProfileView>,

    back_button: Entity<BackButton>,
    profile_button: Entity<ProfileButton>,
    window_buttons: Vec<Entity<WindowButton>>,
}

impl Root {
    pub fn build(cx: &mut App, _window: &mut Window) -> Entity<Self> {
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
                    _ => set_error_async(&mut cx, Some(err)),
                },
            };
        })
        .detach();

        let _app_events_subscribtion = cx
            .subscribe(&global_state.state, |_model, event, cx| {
                let _ = match event.clone() {
                    AppEvent::AssistantChanged(_id) => {
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
                    AppEvent::EmailFormSubmitted(email) => {
                        let auth = cx.global::<Auth>().clone();

                        cx.spawn(|mut cx| async move {
                            let login_result = auth.login_with_email(&mut cx, email.as_str()).await;

                            match login_result {
                                Ok(user) => {
                                    set_user_async(&mut cx, Some(user));
                                    set_authenticated_async(&mut cx, true);
                                    set_active_view_async(&mut cx, ActiveView::ProfileView);
                                }
                                Err(err) => {
                                    set_error_async(&mut cx, Some(err));
                                }
                            };
                        })
                        .detach();
                    }
                };
            })
            .detach();

        let state = global_state.state.clone();

        let view = cx.new(|cx| {
            let assistant_view = cx.new(|cx| AssistantView::new(cx, &state));
            let error_view = cx.new(|cx| ErrorView::new(cx, &state));
            let login_view = cx.new(|cx| LoginView::new(cx, &state));
            let profile_view = cx.new(|cx| ProfileView::new(cx, &state));

            let back_button = cx.new(|cx| BackButton::new(cx, &state));
            let profile_button = cx.new(|cx| ProfileButton::new(cx, &state));
            let close_button = cx.new(|_cx| WindowButton::new(WindowAction::Close));
            let hide_button = cx.new(|_cx| WindowButton::new(WindowAction::Hide));

            Root {
                assistant_view,
                error_view,
                login_view,
                profile_view,

                back_button,
                profile_button,
                window_buttons: vec![close_button, hide_button],
            }
        });

        view
    }

    // TODO: Move to macros
    fn render_space() -> Div {
        div().flex().flex_grow()
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let title_row = div().flex().flex_row().items_start().p_2();
        let content = div().flex().flex_col().flex_grow().pb_2().px_2();

        let back_button = div().flex().mr_2().child(self.back_button.clone());
        let profile_button = div().flex().mr_1().child(self.profile_button.clone());
        let mut title_buttons = self
            .window_buttons
            .iter()
            .map(|button| div().flex().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        title_buttons.push(Root::render_space());
        // only one button is visible per time
        title_buttons.push(back_button);
        title_buttons.push(profile_button);

        let assistant_view = div().child(self.assistant_view.clone());
        let login_view = div().child(self.login_view.clone());
        let profile_view = div().child(self.profile_view.clone());

        let content = div()
            .on_children_prepainted(move |bounds, window, cx| {
                let content_height: f32 = bounds.iter().map(|b| b.size.height.0).sum();
                window.set_frame(utils::window_bounds(cx, content_height));
            })
            .child(title_row.children(title_buttons))
            .child(content.children([assistant_view, login_view, profile_view])) // only one view is visible per time
            .child(self.error_view.clone());

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .border_color(theme.border)
            .child(content)
    }
}

impl EventEmitter<AppEvent> for Root {}
