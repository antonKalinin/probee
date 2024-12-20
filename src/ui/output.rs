use gpui::*;

use crate::clipboard::Clipboard;
use crate::events::UiEvent;
use crate::state::{ActiveView, State};
use crate::theme::Theme;
use crate::ui::CopyOutputButton;

pub struct Output {
    visible: bool,
    scrolled_to_top: bool,
    scrolled_to_bottom: bool,

    text: String,
    scroll_handle: ScrollHandle,

    copy_button: View<CopyOutputButton>,
}

const HINT_TEXT: &str = "Please, copy some text and press CMD + I";
const MAX_HEIGHT: f32 = 320.0;

impl Output {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let error = model.read(cx).error.is_some();
            let loading = model.read(cx).loading;

            this.text = model.read(cx).output.clone();
            this.visible =
                model.read(cx).active_view == ActiveView::AssitantView && !loading && !error;
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
            scrolled_to_top: true,
            scrolled_to_bottom: true,

            copy_button,
            scroll_handle: ScrollHandle::new(),
        };

        // let scroll_handle = this.scroll_handle.clone();

        // cx.spawn(|this, mut cx| async move {
        //     let scroll_epsilon = 4.0;

        //     loop {
        //         let scroll_y: f32 = scroll_handle.offset().y.into();
        //         let height: f32 = scroll_handle.bounds().size.height.into();

        //         if let Some(child_bounds) = scroll_handle.bounds_for_item(0) {
        //             let child_height: f32 = child_bounds.size.height.into();

        //             if child_height <= height {
        //                 continue;
        //             }

        //             let scrolled_to_top = -scroll_y <= 0. + scroll_epsilon;
        //             let scrolled_to_bottom = -scroll_y + height >= child_height - scroll_epsilon;

        //             let _ = this.update(&mut cx, |this, cx| {
        //                 if this.scrolled_to_top != scrolled_to_top
        //                     || this.scrolled_to_bottom != scrolled_to_bottom
        //                 {
        //                     this.scrolled_to_top = scrolled_to_top;
        //                     this.scrolled_to_bottom = scrolled_to_bottom;
        //                     cx.notify();
        //                 }
        //             });
        //         }

        //         cx.background_executor()
        //             .timer(Duration::from_millis(100))
        //             .await;
        //     }
        // })
        // .detach();

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

        let gradient_top = if !self.scrolled_to_top {
            div().absolute().top_0().w_full().h_10().bg(linear_gradient(
                180.,
                linear_color_stop(theme.background, 0.),
                linear_color_stop(theme.background.opacity(0.), 1.),
            ))
        } else {
            div()
        };

        let gradient_bottom = if !self.scrolled_to_bottom {
            let grad = div().absolute().bottom_0().w_full().h_10();
            grad.bg(linear_gradient(
                0.,
                linear_color_stop(theme.background, 0.),
                linear_color_stop(theme.background.opacity(0.), 1.),
            ))
        } else {
            div()
        };

        let output = div()
            .id("output") // element becomes stateful only after assigning ElementId
            .w_full()
            .max_h(px(MAX_HEIGHT))
            .overflow_y_scroll()
            .text_color(theme.text)
            .line_height(theme.line_height)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(self.text.clone())
            .track_scroll(&self.scroll_handle)
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
            .relative()
            .px_1()
            .mt_2()
            .mb_1()
            .w_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .relative()
                    .child(output)
                    .child(gradient_top)
                    .child(gradient_bottom),
            )
            .child(output_actions)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for Output {}
