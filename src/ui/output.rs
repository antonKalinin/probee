use gpui::*;

use crate::clipboard::Clipboard;
use crate::events::UiEvent;
use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::CopyOutputButton;

pub struct Output {
    visible: bool,
    text: String,

    copy_button: View<CopyOutputButton>,
}

const HINT_TEXT: &str = "Please, copy some text and press CMD + I";

impl Output {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let loading = model.read(cx).loading;

            this.text = model.read(cx).output.clone();
            this.visible = model.read(cx).active_view == ActiveView::AssitantView && !loading;
            cx.notify();
        })
        .detach();

        let copy_button = cx.new_view(|cx| CopyOutputButton::new(cx, &state));

        cx.subscribe(&copy_button, move |subscriber, _emitter, event, cx| {
            if UiEvent::CopyOutput == *event && !subscriber.text.is_empty() {
                let clipboard = cx.global_mut::<Clipboard>();
                clipboard.set_text(subscriber.text.clone());
            }
        })
        .detach();

        let this = Output {
            visible: false,
            text: HINT_TEXT.to_string(),

            copy_button,
        };

        this
    }
}

impl Render for Output {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        if self.text.is_empty() {
            return div()
                .flex()
                .flex_col()
                .mt_2()
                .w_full()
                .h_16()
                .items_center()
                .justify_center()
                .text_color(theme.subtext)
                .text_size(theme.text_size)
                .child(HINT_TEXT.to_string())
                .into_any_element();
        }

        let output = div()
            .id("output")
            .w_full()
            .max_h_80()
            .overflow_y_scroll()
            .text_color(theme.text)
            .line_height(theme.line_height)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(self.text.clone())
            .into_any_element();

        let output_actions = div().flex().flex_row().mt_1().justify_end().children(
            vec![self.copy_button.clone()]
                .iter()
                .map(|button| div().flex().ml_2().child(button.clone()))
                .collect::<Vec<_>>(),
        );

        div()
            .flex()
            .flex_col()
            .px_1()
            .mt_2()
            .mb_1()
            .w_full()
            .child(output)
            .child(output_actions)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for Output {}
