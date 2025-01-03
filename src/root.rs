use gpui::*;

use crate::api::*;
use crate::assistant::*;
use crate::events::*;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::*;
use crate::window::Window;

pub struct Root {
    assistant_ids: Vec<String>,

    error_view: View<ErrorView>,
    intro_view: View<Intro>,
    output_view: View<Output>,
    loading_view: View<Loading>,

    app_button: View<AppButton>,
    assistant_buttons: Vec<View<AssistantButton>>,
    window_buttons: Vec<View<WindowButton>>,
}

impl Root {
    pub fn build(wcx: &mut WindowContext) -> View<Self> {
        let state_controler = wcx.global::<StateController>().clone();

        let _app_events_subscribtion = wcx
            .subscribe(&state_controler.model, |_model, event, cx| {
                let _ = match event.clone() {
                    AppEvent::InputUpdated(input) => {
                        let assistant = cx.global::<Assistant>().clone();
                        let assistant_config = get_active_assistant(cx);

                        if assistant_config.is_none() {
                            // TODO: Show an error
                            return;
                        }

                        cx.spawn(|mut cx| async move {
                            let output = assistant.request(&input, assistant_config.unwrap()).await;

                            StateController::update_async(
                                |this, cx| {
                                    this.set_loading(cx, false);

                                    let _ = match output {
                                        Ok(text) => this.set_output(cx, text),
                                        Err(err) => this.set_error(cx, Some(err)),
                                    };
                                },
                                &mut cx,
                            );
                        })
                        .detach();
                    }
                };
            })
            .detach();

        let state = state_controler.model.clone();

        let view = wcx.new_view(|cx| {
            let api = cx.global::<Api>().clone();

            let intro_view = cx.new_view(|cx| Intro::new(cx, &state));
            let output_view = cx.new_view(|cx| Output::new(cx, &state));
            let loading_view = cx.new_view(|cx| Loading::new(cx, &state));
            let error_view = cx.new_view(|cx| ErrorView::new(cx, &state));

            let app_button = cx.new_view(|cx| AppButton::new(cx, &state));
            let close_button = cx.new_view(|_cx| WindowButton::new(WindowAction::Close));
            let hide_button = cx.new_view(|_cx| WindowButton::new(WindowAction::Hide));

            let _ = cx
                .observe(&state, move |this: &mut Root, state: Model<State>, cx| {
                    let state_assistant_ids = state
                        .read(cx)
                        .assistants
                        .iter()
                        .map(|a| a.id.clone())
                        .collect::<Vec<_>>();

                    if state_assistant_ids != this.assistant_ids {
                        let assistants = state.read(cx).assistants.clone();

                        this.assistant_ids = state_assistant_ids;
                        this.assistant_buttons = Root::build_assistant_buttons(assistants, cx);
                        cx.notify();
                    }
                })
                .detach();

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

            // loading assistants in the background
            cx.spawn(|_weak_root, mut cx| async move {
                let assistants = api.get_assistants().await;

                StateController::update_async(
                    |this, cx| match assistants {
                        Ok(assistants) => {
                            this.set_assistants(cx, assistants);
                        }
                        Err(err) => {
                            this.set_error(cx, Some(err));
                        }
                    },
                    &mut cx,
                );
            })
            .detach();

            Root {
                // raw data
                assistant_ids: vec![],

                // views
                intro_view,
                error_view,
                output_view,
                loading_view,

                // buttons
                app_button,
                assistant_buttons: vec![],
                window_buttons: vec![close_button, hide_button],
            }
        });

        view
    }

    fn build_assistant_buttons(
        assistants: Vec<AssistantConfig>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<View<AssistantButton>> {
        let assistant_buttons = assistants
            .iter()
            .map(|assistant| cx.new_view(|cx| AssistantButton::new(cx, assistant.clone(), false)))
            .collect::<Vec<_>>();

        assistant_buttons.iter().for_each(|button| {
            cx.subscribe(button, move |_subscriber, _emitter, event, cx| {
                if let UiEvent::ChangeAssistant(id) = event {
                    set_error(cx, None);
                    set_active_assistant_id(cx, Some(id.clone()));
                    set_active_view(cx, ActiveView::AssitantView);
                }
            })
            .detach();
        });

        assistant_buttons
    }

    // TODO: Move to macros
    fn render_space() -> Div {
        div().flex().flex_grow()
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let actions_row = div().flex().flex_row().flex_wrap().mb_2();
        let content_col = div().flex().flex_col().flex_grow();
        let title_row = div().flex().flex_row().items_start();

        let app_button = div().flex().child(self.app_button.clone());
        let mut title_buttons = self
            .window_buttons
            .iter()
            .map(|button| div().flex().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        title_buttons.push(Root::render_space());
        title_buttons.push(app_button);

        let mut assistant_buttons = self
            .assistant_buttons
            .iter()
            .map(|button| div().flex().mt_2().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        assistant_buttons.push(Root::render_space());

        let handle_size_measured = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_content_size(cx, size), cx);
        };

        let intro = div().child(self.intro_view.clone());
        let error = div().child(self.error_view.clone());
        let output = div().child(self.output_view.clone());
        let loading = div().child(self.loading_view.clone());

        let dynamic_height_content = div()
            .child(title_row.children(title_buttons))
            .child(actions_row.children(assistant_buttons))
            .child(content_col.children([intro, loading, error, output]));

        div()
            .size_full()
            .flex()
            .flex_col()
            .p_2()
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
