use gpui::*;

use crate::state::{ActiveView, State};

use super::assistant_selector::AssistantSelector;
use super::footer::Footer;
use super::loading::Loading;
use super::output::Output;

pub struct AssistantView {
    assistant_selector_view: Entity<AssistantSelector>,
    footer_view: Entity<Footer>,
    loading_view: Entity<Loading>,
    output_view: Entity<Output>,

    visible: bool,
}

impl AssistantView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == ActiveView::AssitantView;
            cx.notify();
        })
        .detach();

        let assistant_selector_view = cx.new(|cx| AssistantSelector::new(cx, &state));
        let footer_view = cx.new(|cx| Footer::new(cx, &state));
        let loading_view = cx.new(|cx| Loading::new(cx, &state));
        let output_view = cx.new(|cx| Output::new(cx, &state));

        AssistantView {
            assistant_selector_view,
            footer_view,
            loading_view,
            output_view,

            visible: true,
        }
    }
}

impl Render for AssistantView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let assistant_selector = div().pb_2().child(self.assistant_selector_view.clone());
        let content_col = div().flex().flex_col().flex_grow().pb_2();
        let output = div().child(self.output_view.clone());
        let loading = div().child(self.loading_view.clone());

        div()
            .child(assistant_selector)
            .child(content_col.children([loading, output]))
            .child(self.footer_view.clone())
            .into_any_element()
    }
}
