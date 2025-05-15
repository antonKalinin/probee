use gpui::*;

use crate::state::settings::*;
use crate::ui::Theme;

use super::components::HotkeyInput;

pub struct ShortcutsView {
    assistant_hotkey_input: Entity<HotkeyInput>,
    visibility_hotkey_input: Entity<HotkeyInput>,
    prev_assistant_hotkey_input: Entity<HotkeyInput>,
    next_assistant_hotkey_input: Entity<HotkeyInput>,

    visible: bool,
}

const VIEW_HEIGHT: f32 = 280.0;

impl ShortcutsView {
    pub fn new(state: &Entity<SettingsState>, cx: &mut Context<Self>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::Shortcuts;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::Shortcuts;
            cx.notify();
        })
        .detach();

        ShortcutsView {
            assistant_hotkey_input: cx.new(|cx| HotkeyInput::new(None, &state, cx)),
            visibility_hotkey_input: cx.new(|cx| HotkeyInput::new(None, &state, cx)),
            prev_assistant_hotkey_input: cx.new(|cx| HotkeyInput::new(None, &state, cx)),
            next_assistant_hotkey_input: cx.new(|cx| HotkeyInput::new(None, &state, cx)),

            visible,
        }
    }
}

impl Render for ShortcutsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        if !self.visible {
            return div().into_any_element();
        }

        let row = || div().w_full().flex().flex_row().mb_6().items_center();

        let label = |text: &str| {
            div()
                .w(px(168.))
                .text_align(TextAlign::Right)
                .text_size(theme.subtext_size)
                .text_color(theme.muted_foreground)
                .font_weight(FontWeight::MEDIUM)
                .mr_6()
                .child(text.to_owned())
        };

        let value = || div().w(px(280.));

        // let separator = || div().w_full().border_b_1().border_color(theme.border);

        div()
            .w_full()
            .h(px(VIEW_HEIGHT))
            .py_8()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(row().children(vec![
                label("Run Assistant"),
                value().child(self.assistant_hotkey_input.clone()),
            ]))
            .child(row().children(vec![
                label("Next Assistant"),
                value().child(self.next_assistant_hotkey_input.clone()),
            ]))
            .child(row().children(vec![
                label("Previous Assistant"),
                value().child(self.prev_assistant_hotkey_input.clone()),
            ]))
            .child(row().children(vec![
                label("Toogle Visibility"),
                value().child(self.visibility_hotkey_input.clone()),
            ]))
            .into_any_element()
    }
}
