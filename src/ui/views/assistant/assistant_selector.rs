use gpui::*;
use std::time::Duration;

use crate::api::*;
use crate::events::UiEvent;
use crate::state::*;
use crate::theme::Theme;

use super::assistant_button::AssistantButton;

pub struct AssistantSelector {
    assistant_ids: Vec<String>,
    assistant_buttons: Vec<Entity<AssistantButton>>,

    loading: bool,
}

impl AssistantSelector {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let api = cx.global::<Api>().clone();

        let _ = cx
            .observe(state, |this, model, cx| {
                let state_assistant_ids = model
                    .read(cx)
                    .assistants
                    .iter()
                    .map(|a| a.id.clone())
                    .collect::<Vec<_>>();

                if state_assistant_ids != this.assistant_ids {
                    let assistants = model.read(cx).assistants.clone();

                    this.assistant_ids = state_assistant_ids;
                    this.assistant_buttons =
                        AssistantSelector::build_assistant_buttons(assistants, &model, cx);

                    this.loading = false;
                    cx.notify();
                }
            })
            .detach();

        // loading assistants in the background
        cx.spawn(|weak_view, mut cx| async move {
            let _ = weak_view.update(&mut cx, |this: &mut AssistantSelector, cx| {
                this.loading = true;
                cx.notify();
            });

            let assistants = api.get_public_assistants().await;

            GlobalState::update_async(
                |this, cx| match assistants {
                    Ok(assistants) => {
                        this.set_assistants(cx, assistants.clone());

                        if let Some(first_assistant) = assistants.first() {
                            this.set_active_assistant_id(cx, Some(first_assistant.id.clone()));
                        }
                    }
                    Err(err) => {
                        this.set_error(cx, Some(err));
                    }
                },
                &mut cx,
            );
        })
        .detach();

        AssistantSelector {
            assistant_ids: vec![],
            assistant_buttons: vec![],

            loading: false,
        }
    }

    fn build_assistant_buttons(
        assistants: Vec<AssistantConfig>,
        state: &Entity<State>,
        cx: &mut App,
    ) -> Vec<Entity<AssistantButton>> {
        let assistant_buttons = assistants
            .iter()
            .map(|assistant| cx.new(|cx| AssistantButton::new(cx, assistant.clone(), state)))
            .collect::<Vec<_>>();

        assistant_buttons.iter().for_each(|button| {
            cx.subscribe(button, move |_subscriber, event, cx| {
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
}

impl Render for AssistantSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let assistant_buttons = self
            .assistant_buttons
            .iter()
            .map(|button| div().flex().mt_2().mr_2().child(button.clone()))
            .collect::<Vec<_>>();

        let assistant_placeholders = (0..2)
            .map(|i| {
                div().flex().child(
                    div()
                        .flex()
                        .w(if i == 0 { px(100.) } else { px(136.) })
                        .h_6()
                        .mt_2()
                        .mr_2()
                        .rounded_full()
                        .bg(theme.secondary)
                        .with_animation(
                            "pulsating",
                            Animation::new(Duration::from_secs(1))
                                .repeat()
                                .with_easing(pulsating_between(0.5, 1.0)),
                            |label, delta| label.opacity(delta),
                        ),
                )
            })
            .collect::<Vec<_>>();

        let assistants = if self.loading {
            assistant_placeholders
        } else {
            assistant_buttons
        };

        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .children(assistants)
            .into_any_element()
    }
}

impl EventEmitter<UiEvent> for AssistantSelector {}
