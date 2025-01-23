use async_std::stream::StreamExt;
use gpui::*;

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::services::Auth;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::*;
use crate::window::Window;

pub struct Root {
    assistant_view: View<AssistantView>,
    error_view: View<ErrorView>,
    login_view: View<LoginView>,
    profile_view: View<ProfileView>,

    profile_button: View<ProfileButton>,
    window_buttons: Vec<View<WindowButton>>,
}

impl Root {
    pub fn build(wcx: &mut WindowContext) -> View<Self> {
        let state_controler = wcx.global::<StateController>().clone();

        let _app_events_subscribtion = wcx
            .subscribe(&state_controler.model, |_model, event, cx| {
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

        let state = state_controler.model.clone();

        let view = wcx.new_view(|cx| {
            let assistant_view = cx.new_view(|cx| AssistantView::new(cx, &state));
            let error_view = cx.new_view(|cx| ErrorView::new(cx, &state));
            let login_view = cx.new_view(|cx| LoginView::new(cx, &state));
            let profile_view = cx.new_view(|cx| ProfileView::new(cx, &state));

            let profile_button = cx.new_view(|cx| ProfileButton::new(cx, &state));
            let close_button = cx.new_view(|_cx| WindowButton::new(WindowAction::Close));
            let hide_button = cx.new_view(|_cx| WindowButton::new(WindowAction::Hide));

            cx.subscribe(&close_button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::CloseWindow = event {
                    cx.quit();
                }
            })
            .detach();

            cx.subscribe(&hide_button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::HideWindow = event {
                    Window::hide(cx);
                }
            })
            .detach();

            cx.subscribe(&profile_button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::ChangeActiveView(view) = event {
                    set_active_view(cx, view.clone());
                    set_error(cx, None);
                }
            })
            .detach();

            Root {
                assistant_view,
                error_view,
                login_view,
                profile_view,

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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let title_row = div().flex().flex_row().items_start().p_2();
        let content = div().flex().flex_col().flex_grow().pb_2().px_2();

        let profile_button = div().flex().mr_1().child(self.profile_button.clone());
        let mut title_buttons = self
            .window_buttons
            .iter()
            .map(|button| div().flex().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        title_buttons.push(Root::render_space());
        // TODO: show if not auithenticated
        title_buttons.push(profile_button);

        let handle_size_measured = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_content_size(cx, size), cx);
        };

        let assistant_view = div().child(self.assistant_view.clone());
        let login_view = div().child(self.login_view.clone());
        let profile_view = div().child(self.profile_view.clone());

        let dynamic_height_content = div()
            .child(title_row.children(title_buttons))
            .child(content.children([assistant_view, login_view, profile_view])) // only one view is visible per time
            .child(self.error_view.clone());

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .border_color(theme.border)
            .child(
                size_observer()
                    .on_size_measured(handle_size_measured)
                    .child(dynamic_height_content),
            )
    }
}

impl EventEmitter<AppEvent> for Root {}
