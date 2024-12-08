use gpui::*;

use crate::assistant::AssistMode;
use crate::events::UiEvent;
use crate::state::StateController;
use crate::theme::Theme;
use crate::views::*;

pub struct Root {
    mode_buttons: Vec<View<ModeButton>>,
    output: View<Output>,
}

impl Root {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let state = StateController::init(cx).model;
            let output = cx.new_view(|cx| Output::new(cx, &state));

            let mode_buttons = vec![
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::Translate, true)),
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::Explain, false)),
                cx.new_view(|cx| ModeButton::new(cx, AssistMode::GrammarCorrect, false)),
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
                output,
                mode_buttons,
            }
        });

        view
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let content_col = div().flex().flex_col().flex_grow();
        let actions_row = div().flex().flex_row();
        let space = div().flex().flex_grow();

        let mut mode_buttons = self
            .mode_buttons
            .iter()
            .map(|button| div().flex().mr_1().child(button.clone()))
            .collect::<Vec<_>>();

        mode_buttons.push(space);

        let on_content_sized = |size, cx: &mut WindowContext<'_>| {
            StateController::update(|this, cx| this.set_output_size(cx, size), cx);
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .p_2()
            .bg(theme.background)
            .border_color(theme.border)
            .child(actions_row.children(mode_buttons))
            .child(
                size_observer()
                    .on_sized(on_content_sized)
                    .child(content_col.child(self.output.clone())),
            )
    }
}
