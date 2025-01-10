use async_std::stream::StreamExt;
use gpui::*;

use crate::assistant::*;
use crate::errors::*;
use crate::events::*;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::*;
use crate::window::Window;

pub struct Root {
    assistants_view: View<Assistants>,
    error_view: View<ErrorView>,
    intro_view: View<Intro>,
    output_view: View<Output>,
    loading_view: View<Loading>,

    app_button: View<AppButton>,
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
                };
            })
            .detach();

        let state = state_controler.model.clone();

        let view = wcx.new_view(|cx| {
            let assistants_view = cx.new_view(|cx| Assistants::new(cx, &state));
            let intro_view = cx.new_view(|cx| Intro::new(cx, &state));
            let output_view = cx.new_view(|cx| Output::new(cx, &state));
            let loading_view = cx.new_view(|cx| Loading::new(cx, &state));
            let error_view = cx.new_view(|cx| ErrorView::new(cx, &state));

            let app_button = cx.new_view(|cx| AppButton::new(cx, &state));
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

            cx.subscribe(&app_button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::ChangeActiveView(view) = event {
                    set_active_view(cx, view.clone());
                    set_error(cx, None);
                    set_active_assistant_id(cx, None);
                }
            })
            .detach();

            cx.subscribe(&login_button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::Login = event {
                    cx.spawn(|_weak_root, mut cx| async move {
                        let _background = cx.background_executor().clone();

                        let result =
                            cx.update(|cx| cx.open_url("https://cmdi.app/login?from=native"));

                        println!("Tried to open url: {:?}", result);
                    })
                    .detach();
                }
            })
            .detach();

            Root {
                assistants_view,
                intro_view,
                error_view,
                output_view,
                loading_view,

                app_button,
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

        let app_button = div().flex().ml_2().child(self.app_button.clone());
        let login_button = div().flex().child(self.login_button.clone());
        let mut title_buttons = self
            .window_buttons
            .iter()
            .map(|button| div().flex().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        title_buttons.push(Root::render_space());
        // TODO: show if not auithenticated
        title_buttons.push(login_button);
        title_buttons.push(app_button);

        let handle_size_measured = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_content_size(cx, size), cx);
        };

        let intro = div().child(self.intro_view.clone());
        let output = div().child(self.output_view.clone());
        let loading = div().child(self.loading_view.clone());

        let dynamic_height_content = div()
            .child(title_row.children(title_buttons))
            .child(assistants_row.child(self.assistants_view.clone()))
            .child(content_col.children([intro, loading, output]))
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
