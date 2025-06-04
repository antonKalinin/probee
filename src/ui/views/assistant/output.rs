use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::clipboard::Clipboard;
use crate::events::UiEvent;
use crate::state::app_state::*;
use crate::ui::{Spinner, Theme};

use super::clear_output_button::ClearOutputButton;
use super::copy_output_button::CopyOutputButton;

pub struct Output {
    visible: bool,
    loading: bool,
    text: String,
    description: String,
    actions_visible: bool,

    scroll_handle: ScrollHandle,
    copy_button: Entity<CopyOutputButton>,
    clear_button: Entity<ClearOutputButton>,
}

const MAX_HEIGHT: f32 = 320.0;

impl Output {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
        cx.observe(state, |this, state, cx| {
            let error = state.read(cx).error.is_some();
            let loading = state.read(cx).loading;
            let prompt = get_active_prompt(cx);

            this.text = state.read(cx).output.clone();
            this.description = prompt.map(|a| a.description.clone()).unwrap_or_default();
            this.loading = loading;
            this.visible = state.read(cx).active_view == AppView::AssistantView && !error;
            cx.notify();
        })
        .detach();

        let copy_button = cx.new(|cx| CopyOutputButton::new(cx, &state));
        let clear_button = cx.new(|cx| ClearOutputButton::new(cx, &state));

        cx.subscribe(&copy_button, move |subscriber, _emitter, event, cx| {
            if UiEvent::CopyOutput == *event && !subscriber.text.is_empty() {
                let clipboard = cx.global_mut::<Clipboard>();
                clipboard.set_text(subscriber.text.clone());
            }
        })
        .detach();

        cx.subscribe(&clear_button, move |subscriber, _emitter, event, cx| {
            if UiEvent::ClearOutput == *event && !subscriber.text.is_empty() {
                subscriber.text = "".to_owned();
                set_output(cx, "".to_owned());
                cx.notify();
            }
        })
        .detach();

        Output {
            visible: false,
            loading: false,
            text: "".to_owned(),
            description: "".to_owned(),
            scroll_handle: ScrollHandle::new(),
            actions_visible: false,

            copy_button,
            clear_button,
        }
    }

    fn scroll_gradient_visible(&self) -> [bool; 2] {
        let scroll_epsilon = 4.0;
        let scroll_y: f32 = self.scroll_handle.offset().y.into();
        let view_height: f32 = self.scroll_handle.bounds().size.height.into();

        if let Some(child_bounds) = self.scroll_handle.bounds_for_item(0) {
            let child_height: f32 = child_bounds.size.height.into();

            if child_height <= view_height {
                return [false, false];
            }

            let top_gradient_visible = -scroll_y >= 0. + scroll_epsilon;
            let bottom_gradient_visible = -scroll_y + view_height <= child_height - scroll_epsilon;

            return [top_gradient_visible, bottom_gradient_visible];
        }

        [false, false]
    }

    fn render_loading(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Theme>();

        let spinner = div()
            .flex()
            .child(Spinner::new().size(px(24.)).color(theme.muted_foreground));

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .h_20()
            .w_full()
            .child(spinner)
            .into_any_element()
    }
}

impl Render for Output {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        if self.loading {
            return self.render_loading(cx);
        }

        // Render assisntant description
        if self.text.is_empty() {
            return div()
                .flex()
                .flex_col()
                .mt_2()
                .mb_1()
                .px_1()
                .h_auto()
                .font_weight(FontWeight::LIGHT)
                .text_color(theme.muted_foreground)
                .text_size(theme.subtext_size)
                .child(self.description.clone())
                .into_any_element();
        }

        let [top_gradient_visible, bottom_gradient_visible] = self.scroll_gradient_visible();

        let gradient_top = if top_gradient_visible {
            div().absolute().top_0().w_full().h_10().bg(linear_gradient(
                180.,
                linear_color_stop(theme.background, 0.),
                linear_color_stop(theme.background.opacity(0.), 1.),
            ))
        } else {
            div()
        };

        let gradient_bottom = if bottom_gradient_visible {
            let grad = div().absolute().bottom_0().w_full().h_10();
            grad.bg(linear_gradient(
                0.,
                linear_color_stop(theme.background, 0.),
                linear_color_stop(theme.background.opacity(0.), 1.),
            ))
        } else {
            div()
        };

        let handle_hover = cx.listener({
            move |this, hovered, _window, cx: &mut Context<Self>| {
                this.actions_visible = *hovered;
                cx.notify();
            }
        });

        let output = div()
            .id("output") // element becomes stateful only after assigning ElementId
            .w_full()
            .max_h(px(MAX_HEIGHT))
            .overflow_y_scroll()
            .text_color(theme.foreground)
            .line_height(theme.line_height)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .font_weight(FontWeight::LIGHT)
            .child(self.text.clone())
            .track_scroll(&self.scroll_handle)
            .into_any_element();

        let output_actions = div()
            .mt_1()
            .flex()
            .flex_row()
            .justify_end()
            .gap_3()
            .opacity(0.)
            .when(self.actions_visible, |this| this.opacity(1.0))
            .children(vec![
                div().flex().child(self.copy_button.clone()),
                div().flex().child(self.clear_button.clone()),
            ]);

        div()
            .id("output-container")
            .relative()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .px_1()
            .mt_2()
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
            .on_hover(handle_hover)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for Output {}
