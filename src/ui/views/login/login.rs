use crate::state::{ActiveView, State};
use crate::theme::Theme;
use gpui::*;

pub struct LoginView {
    visible: bool,
}

const TEXT: &str = "\
Please check yor email for the sign in link. \
";

impl LoginView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<State>) -> Self {
        cx.observe(state, |this, model, cx| {
            let data = model.read(cx);
            this.visible = data.active_view == ActiveView::LoginView && !data.authenticated;
            cx.notify();
        })
        .detach();

        LoginView { visible: false }
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        // let text_input = cx.new(|cx| TextInput {
        //     focus_handle: cx.focus_handle(),
        //     content: "".into(),
        //     placeholder: "Type here...".into(),
        //     selected_range: 0..0,
        //     selection_reversed: false,
        //     marked_range: None,
        //     last_layout: None,
        //     last_bounds: None,
        //     is_selecting: false,
        // });

        div()
            .line_height(theme.line_height)
            .w_full()
            .mt_4()
            .px_1()
            .text_color(theme.text)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .font_weight(FontWeight::LIGHT)
            .child(TEXT.to_owned())
            .into_any_element()
    }
}
