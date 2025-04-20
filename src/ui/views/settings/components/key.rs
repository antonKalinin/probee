use gpui::*;

use crate::ui::ActiveTheme;

#[derive(IntoElement)]
pub struct Key {
    pub base: Div,
    text: String,
}

impl From<Key> for AnyElement {
    fn from(key: Key) -> Self {
        key.into_any_element()
    }
}

impl Key {
    pub fn new(text: &str) -> Self {
        Key {
            base: div().flex_shrink_0(),
            text: String::from(text),
        }
    }
}

impl Styled for Key {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Key {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Key {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        self.base
            .flex()
            .h_10()
            .min_w_10()
            .px_2()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .bg(theme.secondary)
            .border_1()
            .border_color(theme.border)
            .shadow_sm()
            .rounded_md()
            .text_size(theme.subtext_size)
            .text_color(theme.muted_foreground)
            .child(self.text)
    }
}
