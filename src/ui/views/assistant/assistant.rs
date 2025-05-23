use gpui::*;

use crate::events::*;
use crate::services::{Api, Storage, StorageKey};
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

        // load prompts in the background
        cx.spawn(async move |weak_view, cx| {
            let _ = weak_view.update(cx, |this: &mut AssistantView, cx| {
                this.loading = true;
                cx.notify();
            });

            let prompts = api.get_prompts(cx).await;
            let saved_propmt_id = storage.get(StorageKey::AssistantId);

            AppStateController::update_async(
                |this, cx| match prompts {
                    Ok(prompts) => {
                        this.set_promts(cx, prompts.clone());
                        let prompts_ids = prompts.iter().map(|a| a.id.clone()).collect::<Vec<_>>();
                        let first_prompt_id = prompts_ids.first().cloned();

                        // ensure if the saved prompt id is still valid
                        let saved_propmt_id = saved_propmt_id
                            .as_ref()
                            .filter(|id| prompts_ids.contains(id))
                            .cloned();

                        match (saved_propmt_id, first_prompt_id) {
                            (Some(id), _) | (None, Some(id)) => {
                                this.set_active_prompt_id(cx, Some(id))
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        set_error(cx, Some(err));
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
