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
    assistants_view: View<Assistants>,
    error_view: View<ErrorView>,
    footer_view: View<Footer>,
    intro_view: View<Intro>,
    loading_view: View<Loading>,
    login_view: View<Login>,
    output_view: View<Output>,

    login_button: View<LoginButton>,
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
                            let result = auth.login_with_email(cx, email.as_str()).await;

                            match result {
                                Ok(_) => {}
                                Err(err) => {
                                    // set_error_async(&mut cx, Some(err));
                                }
                            }
                        })
                        .detach();
                    }
                };
            })
            .detach();

        let state = state_controler.model.clone();

        let view = wcx.new_view(|cx| {
            let assistants_view = cx.new_view(|cx| Assistants::new(cx, &state));
            let error_view = cx.new_view(|cx| ErrorView::new(cx, &state));
            let footer_view = cx.new_view(|cx| Footer::new(cx, &state));
            let intro_view = cx.new_view(|cx| Intro::new(cx, &state));
            let loading_view = cx.new_view(|cx| Loading::new(cx, &state));
            let login_view = cx.new_view(|cx| Login::new(cx, &state));
            let output_view = cx.new_view(|cx| Output::new(cx, &state));

            let login_button = cx.new_view(|cx| LoginButton::new(cx, &state));
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
                    Window::toggle(cx);
                }
            })
            .detach();

            cx.subscribe(&login_button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::ChangeActiveView(view) = event {
                    set_active_view(cx, view.clone());
                    set_error(cx, None);
                    set_active_assistant_id(cx, None);
                }
            })
            .detach();

            Root {
                assistants_view,
                error_view,
                footer_view,
                intro_view,
                loading_view,
                login_view,
                output_view,

                login_button,
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
        let assistants_row = div().pb_2().px_2();
        let content_col = div().flex().flex_col().flex_grow().pb_2().px_2();

        let login_button = div().flex().mr_1().child(self.login_button.clone());
        let mut title_buttons = self
            .window_buttons
            .iter()
            .map(|button| div().flex().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        title_buttons.push(Root::render_space());
        // TODO: show if not auithenticated
        title_buttons.push(login_button);

        let handle_size_measured = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_content_size(cx, size), cx);
        };

        let intro = div().child(self.intro_view.clone());
        let login = div().child(self.login_view.clone());
        let output = div().child(self.output_view.clone());
        let loading = div().child(self.loading_view.clone());

        let dynamic_height_content = div()
            .child(title_row.children(title_buttons))
            .child(assistants_row.child(self.assistants_view.clone()))
            .child(content_col.children([intro, login, loading, output]))
            .child(self.error_view.clone())
            .child(self.footer_view.clone());

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
