use gpui::*;

use crate::state::settings::*;
use crate::ui::Theme;

pub struct HotkeyInput {
    keystroke: Option<String>,
}

impl HotkeyInput {
    pub fn new(
        keystroke: Option<String>,
        _state: &Entity<SettingsState>,
        _cx: &mut Context<Self>,
    ) -> Self {
        HotkeyInput { keystroke }
    }
}

impl Render for HotkeyInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |_this, _event, _window, cx: &mut Context<Self>| {
                println!("Hotkey input clicked");
            }
        });

        div()
            .w_full()
            .h_8()
            .p_1()
            .gap_1()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .bg(theme.muted)
            .text_color(theme.muted_foreground)
            .rounded_lg()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, on_click)
            .child("Record Hotkey")
    }
}
