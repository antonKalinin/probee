use gpui::*;

use crate::assistant::Prompt;
use crate::events::*;
use crate::state::app_state::*;
use crate::ui::*;

pub struct LibraryView {
    active_prompt_id: Option<String>,
    header_view: Entity<Header>,
    prompts: Vec<Prompt>,
    visible: bool,
}

impl LibraryView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<AppState>) -> Self {
        let header_view = cx.new(|cx| Header::new(cx, &state));

        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == AppView::LibraryView;
            this.prompts = model.read(cx).prompts.clone();
            this.active_prompt_id = model.read(cx).active_prompt_id.clone();

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

            prompts: state.read(cx).prompts.clone(),
            active_prompt_id: state.read(cx).active_prompt_id.clone(),
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

        let assistant_items = self.prompts.iter().map(|assistant| {
            let propmt_id = assistant.id.clone();

            let bg_color = match &self.active_prompt_id {
                Some(active_prompt_id) if active_prompt_id == &propmt_id => theme.accent,
                _ => theme.background,
            };

            let on_click = cx.listener(move |_this, _event, _window, cx: &mut Context<Self>| {
                set_active_prompt_id(cx, Some(propmt_id.clone()));
                set_active_view(cx, AppView::AssistantView);
                set_output(cx, "".to_owned());
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
                        .text_color(theme.primary)
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
