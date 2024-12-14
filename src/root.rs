use gpui::*;

use crate::assistant::AssistMode;
use crate::events::UiEvent;
use crate::state::StateController;
use crate::theme::Theme;
use crate::views::*;

pub struct Root {
    output_view: View<Output>,
    loading_view: View<Loading>,
    // error_view: View<Error>,
    app_button: View<AppButton>,
    mode_buttons: Vec<View<ModeButton>>,
    window_buttons: Vec<View<WindowButton>>,
}

impl Root {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let state = StateController::init(cx).model;
            let output_view = cx.new_view(|cx| Output::new(cx, &state));
            let loading_view = cx.new_view(|cx| Loading::new(cx, &state));

            let app_button = cx.new_view(|cx| AppButton::new(cx, &state));

            let mode_buttons = vec![
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::Translate, false)),
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::TranslateWordByWord, false)),
            ];

            let window_buttons = vec![
                cx.new_view(|_cx| WindowButton::new(WindowAction::Close)),
                cx.new_view(|_cx| WindowButton::new(WindowAction::Hide)),
            ];

            mode_buttons.iter().for_each(|button| {
                cx.subscribe(button, move |_subscriber, _emitter, event, cx| {
                    if let UiEvent::ModeChanged(mode) = event {
                        StateController::update(|this, cx| this.set_mode(cx, mode.clone()), cx);
                    }
                })
                .detach();
            });

            Root {
                output_view,
                loading_view,
                app_button,
                mode_buttons,
                window_buttons,
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

        let actions_row = div().flex().flex_row();
        let content_col = div().flex().flex_col().flex_grow();
        let title_row = div().flex().flex_row().items_start().mb_2();

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
            .map(|button| div().flex().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        mode_buttons.push(Root::render_space());

        let on_content_sized = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_output_size(cx, size), cx);
        };

        let output = div().child(self.output_view.clone());
        let loading = div().child(self.loading_view.clone());
        // TODO: Add error view

        div()
            .size_full()
            .flex()
            .flex_col()
            .p_2()
            .bg(theme.background)
            .border_color(theme.border)
            .child(title_row.children(title_buttons))
            .child(actions_row.children(mode_buttons))
            .child(
                size_observer()
                    .on_sized(on_content_sized)
                    .child(content_col.children([loading, output])),
            )
    }
}
