use gpui::*;

use crate::state::{ActiveView, State};

use crate::events::*;
use crate::services::AssistantConfig;
use crate::state::*;
use crate::ui::*;

use super::item::Item;

pub struct LibraryView {
    header_view: Entity<Header>,

    assistants: Vec<AssistantConfig>,
    visible: bool,
    loading: bool,
}

impl LibraryView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        let header_view = cx.new(|cx| Header::new(cx, &state));

        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == ActiveView::LibraryView;
            this.assistants = model.read(cx).assistants.clone();

            cx.notify();
        })
        .detach();

        cx.subscribe(&header_view, move |_subscriber, _emitter, event, cx| {
            if UiEvent::ToggleAssistantLibrary == *event {
                set_active_view(cx, ActiveView::AssistantView);
            }
        })
        .detach();

        LibraryView {
            header_view,

            assistants: state.read(cx).assistants.clone(),
            visible: false,
            loading: false,
        }
    }
}

impl Render for LibraryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let header = div().child(self.header_view.clone());
        let assistant_list = || div().px_1().my_2();
        let assistant_items = self.assistants.iter().map(|assistant| {
            let item = cx.new(|cx| Item::new(cx, assistant.clone()));

            div().py_2().child(item)
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
