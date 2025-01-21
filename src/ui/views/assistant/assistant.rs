use gpui::*;

use crate::state::{ActiveView, State};

use super::assistant_selector::AssistantSelector;
use super::footer::Footer;
use super::loading::Loading;
use super::output::Output;

pub struct AssistantView {
    assistant_selector_view: View<AssistantSelector>,
    footer_view: View<Footer>,
    loading_view: View<Loading>,
    output_view: View<Output>,

    visible: bool,
}

impl AssistantView {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.visible = model.read(cx).active_view == ActiveView::AssitantView;
            cx.notify();
        })
        .detach();

        let assistant_selector_view = cx.new_view(|cx| AssistantSelector::new(cx, &state));
        let footer_view = cx.new_view(|cx| Footer::new(cx, &state));
        let loading_view = cx.new_view(|cx| Loading::new(cx, &state));
        let output_view = cx.new_view(|cx| Output::new(cx, &state));

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
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
