use gpui::*;

use crate::assistant::AssistMode;
use crate::events::UiEvent;
use crate::state::{ActiveView, StateController};
use crate::theme::Theme;
use crate::ui::*;
use crate::window::Window;

pub struct Root {
    error_view: View<ErrorView>,
    intro_view: View<Intro>,
    output_view: View<Output>,
    loading_view: View<Loading>,

    app_button: View<AppButton>,
    mode_buttons: Vec<View<ModeButton>>,
    window_buttons: Vec<View<WindowButton>>,
}

impl Root {
    pub fn build(wcx: &mut WindowContext) -> View<Self> {
        let view = wcx.new_view(|cx| {
            let state = StateController::init(cx).model;
            let intro_view = cx.new_view(|cx| Intro::new(cx, &state));
            let output_view = cx.new_view(|cx| Output::new(cx, &state));
            let loading_view = cx.new_view(|cx| Loading::new(cx, &state));
            let error_view = cx.new_view(|cx| ErrorView::new(cx, &state));

            let app_button = cx.new_view(|cx| AppButton::new(cx, &state));
            let close_button = cx.new_view(|_cx| WindowButton::new(WindowAction::Close));
            let hide_button = cx.new_view(|_cx| WindowButton::new(WindowAction::Hide));

            let mode_buttons = vec![
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::Translate, false)),
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::WordMorphology, false)),
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::PlainFinnish, false)),
            ];

            mode_buttons.iter().for_each(|button| {
                cx.subscribe(button, move |_subscriber, _emitter, event, cx| {
                    if let UiEvent::ChangeMode(mode) = event {
                        let view = ActiveView::AssitantView;
                        StateController::update(
                            |this, cx| this.set_mode(cx, Some(mode.clone())),
                            cx,
                        );
                        StateController::update(|this, cx| this.set_active_view(cx, view), cx);
                    }
                })
                .detach();
            });

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
                    StateController::update(|this, cx| this.set_active_view(cx, view.clone()), cx);
                    StateController::update(|this, cx| this.set_mode(cx, None), cx);
                }
            })
            .detach();

            Root {
                intro_view,
                error_view,
                output_view,
                loading_view,

                app_button,
                mode_buttons,
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

        let mut mode_buttons = self
            .mode_buttons
            .iter()
            .map(|button| div().flex().mt_2().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        mode_buttons.push(Root::render_space());

        let handle_size_measured = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_view_size(cx, size), cx);
        };

        let intro = div().child(self.intro_view.clone());
        let error = div().child(self.error_view.clone());
        let output = div().child(self.output_view.clone());
        let loading = div().child(self.loading_view.clone());

        let dynamic_height_content = div()
            .child(title_row.children(title_buttons))
            .child(actions_row.children(mode_buttons))
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
