use crate::state::State;
use crate::theme::Theme;
use gpui::*;

pub struct Loading {
    active: bool,
}

impl Loading {
    pub fn new(cx: &mut ViewContext<Self>, state: &Model<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            this.active = model.read(cx).loading.clone();
            cx.notify();
        })
        .detach();

        Loading { active: false }
    }
}

impl Render for Loading {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let text = if self.active { "Loading..." } else { "" };

        div().child(text)
    }
}
