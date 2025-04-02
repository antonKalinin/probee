use gpui::*;

use crate::events::*;
use crate::services::{Api, Storage};
use crate::state::*;
use crate::ui::*;

use super::output::Output;

pub struct AssistantView {
    header_view: Entity<Header>,
    output_view: Entity<Output>,

    visible: bool,
    loading: bool,
}

impl AssistantView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let api = cx.global::<Api>().clone();
        let storage = cx.global::<Storage>().clone();

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

        // load assistants in the background
        cx.spawn(async move |weak_view, cx| {
            let _ = weak_view.update(cx, |this: &mut AssistantView, cx| {
                this.loading = true;
                cx.notify();
            });

            let assistants = api.get_assistants(cx).await;
            let saved_assistant_id = storage.get("assistant_id".into());

            GlobalState::update_async(
                |this, cx| match assistants {
                    Ok(assistants) => {
                        this.set_assistants(cx, assistants.clone());
                        let assistant_ids =
                            assistants.iter().map(|a| a.id.clone()).collect::<Vec<_>>();
                        let first_assistant_id = assistant_ids.first().cloned();

                        // ensure if the saved assistant id is still valid
                        let saved_assistant_id = saved_assistant_id
                            .as_ref()
                            .filter(|id| assistant_ids.contains(id))
                            .cloned();

                        match (saved_assistant_id, first_assistant_id) {
                            (Some(id), _) | (None, Some(id)) => {
                                this.set_active_assistant_id(cx, Some(id))
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        this.set_error(cx, Some(err));
                    }
                },
                cx,
            );

            let _ = weak_view.update(cx, |this: &mut AssistantView, cx| {
                this.loading = false;
                cx.notify();
            });
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

        let assistant_header = div().child(self.header_view.clone());
        let output = div().child(self.output_view.clone());

        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .child(assistant_header)
            .child(output)
            .into_any_element()
    }
}
