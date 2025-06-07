use gpui::*;

use crate::events::*;
use crate::state::app_state::*;
use crate::ui::*;

use super::output::Output;

pub struct AssistantView {
    header_view: Entity<Header>,
    output_view: Entity<Output>,

    visible: bool,
    loading: bool,
}

impl AssistantView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
        let header_view = cx.new(|cx| Header::new(cx, &state));
        let output_view = cx.new(|cx| Output::new(cx, &state));

        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == AppView::AssistantView;
            cx.notify();
        })
        .detach();

        cx.subscribe(&header_view, move |_subscriber, _emitter, event, cx| {
            if UiEvent::ToggleAssistantLibrary == *event {
                set_active_view(cx, AppView::LibraryView);
            }
        })
        .detach();

        AssistantView {
            header_view,
            output_view,

            visible: true,
            loading: false,
        }
    }
}

impl Render for AssistantView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        if self.loading {
            // 3 lines of skeleton: header + 2 lines of output
            return div()
                .flex()
                .flex_col()
                .flex_shrink_0()
                .child(div().w_2_3().mb_2().child(Skeleton::new()))
                .child(div().w_full().mb_2().child(Skeleton::new()))
                .child(div().w_4_5().child(Skeleton::new()))
                .into_any_element();
        }

        let prompt_header = div().child(self.header_view.clone());
        let output = div().child(self.output_view.clone());

        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .child(prompt_header)
            .child(output)
            .into_any_element()
    }
}
