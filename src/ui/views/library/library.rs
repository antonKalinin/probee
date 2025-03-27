use gpui::*;

use crate::state::{AppView, State};

use crate::events::*;
use crate::services::AssistantConfig;
use crate::state::*;
use crate::ui::*;

pub struct LibraryView {
    header_view: Entity<Header>,

    assistants: Vec<AssistantConfig>,
    active_assistant_id: Option<String>,
    visible: bool,
}

impl LibraryView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let header_view = cx.new(|cx| Header::new(cx, &state));

        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == AppView::LibraryView;
            this.assistants = model.read(cx).assistants.clone();
            this.active_assistant_id = model.read(cx).active_assistant_id.clone();

            cx.notify();
        })
        .detach();

        cx.subscribe(&header_view, move |_subscriber, _emitter, event, cx| {
            if UiEvent::ToggleAssistantLibrary == *event {
                set_active_view(cx, AppView::AssistantView);
            }
        })
        .detach();

        LibraryView {
            header_view,

            assistants: state.read(cx).assistants.clone(),
            active_assistant_id: state.read(cx).active_assistant_id.clone(),
            visible: false,
        }
    }
}

impl Render for LibraryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let header = div().child(self.header_view.clone());
        let assistant_list = || div().my_2();

        let assistant_items = self.assistants.iter().map(|assistant| {
            let assistant_id = assistant.id.clone();

            let bg_color = match &self.active_assistant_id {
                Some(active_assistant_id) if active_assistant_id == &assistant_id => theme.accent,
                _ => theme.background,
            };

            let on_click = cx.listener(move |_this, _event, _window, cx: &mut Context<Self>| {
                set_active_assistant_id(cx, Some(assistant_id.clone()));
                set_active_view(cx, AppView::AssistantView);
            });

            div()
                .flex()
                .flex_col()
                .p_3()
                .rounded_sm()
                .bg(bg_color)
                .hover(|style| style.bg(theme.muted))
                .child(
                    div()
                        .text_size(theme.text_size)
                        .font_weight(FontWeight::MEDIUM)
                        .child(assistant.name.clone()),
                )
                .child(
                    div()
                        .text_size(theme.subtext_size)
                        .text_color(theme.muted_foreground)
                        .child(assistant.description.clone()),
                )
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, on_click)
                .into_any_element()
        });

        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .child(header)
            .child(assistant_list().children(assistant_items))
            .into_any_element()
    }
}
