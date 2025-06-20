use gpui::*;

use crate::services::{HotKey, Storage, StorageKey};
use crate::state::settings_state::*;
use crate::ui::Theme;

use super::components::HotkeyInput;

pub struct HotkeysView {
    assistant_hotkey_input: Entity<HotkeyInput>,
    visibility_hotkey_input: Entity<HotkeyInput>,
    prev_assistant_hotkey_input: Entity<HotkeyInput>,
    next_assistant_hotkey_input: Entity<HotkeyInput>,

    visible: bool,
}

const VIEW_HEIGHT: f32 = 280.0;

impl HotkeysView {
    pub fn new(state: &Entity<SettingsState>, cx: &mut Context<Self>) -> Self {
        let storage = cx.global::<Storage>();
        let state_data = state.read(cx);
        let visible = state_data.active_tab == SettingsTabType::Hotkeys;

        let assistant_hotkey = storage
            .get(StorageKey::HotkeyRunAssistant)
            .map(|s| HotKey::from_keystroke(s.as_str()).ok())
            .unwrap_or(None);

        let visibility_hotkey = storage
            .get(StorageKey::HotkeyToogleVisibility)
            .map(|s| HotKey::from_keystroke(s.as_str()).ok())
            .unwrap_or(None);

        let prev_assistant_hotkey = storage
            .get(StorageKey::HotkeyPrevPropmt)
            .map(|s| HotKey::from_keystroke(s.as_str()).ok())
            .unwrap_or(None);

        let next_assistant_hotkey = storage
            .get(StorageKey::HotkeyNextPrompt)
            .map(|s| HotKey::from_keystroke(s.as_str()).ok())
            .unwrap_or(None);

        let assistant_hotkey_cb = Box::new(|hotkey: HotKey, cx: &mut Context<_>| {
            let _ = cx
                .global::<Storage>()
                .set(StorageKey::HotkeyRunAssistant, hotkey.to_keystroke());
        });

        let visibility_hotkey_cb = Box::new(|hotkey: HotKey, cx: &mut Context<_>| {
            let _ = cx
                .global::<Storage>()
                .set(StorageKey::HotkeyToogleVisibility, hotkey.to_keystroke());
        });

        let prev_hotkey_cb = Box::new(|hotkey: HotKey, cx: &mut Context<_>| {
            let _ = cx
                .global::<Storage>()
                .set(StorageKey::HotkeyPrevPropmt, hotkey.to_keystroke());
        });

        let next_hotkey_cb = Box::new(|hotkey: HotKey, cx: &mut Context<_>| {
            let _ = cx
                .global::<Storage>()
                .set(StorageKey::HotkeyNextPrompt, hotkey.to_keystroke());
        });

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::Hotkeys;
            cx.notify();
        })
        .detach();

        HotkeysView {
            assistant_hotkey_input: cx
                .new(|cx| HotkeyInput::new(assistant_hotkey, assistant_hotkey_cb, cx)),
            visibility_hotkey_input: cx
                .new(|cx| HotkeyInput::new(visibility_hotkey, visibility_hotkey_cb, cx)),
            prev_assistant_hotkey_input: cx
                .new(|cx| HotkeyInput::new(prev_assistant_hotkey, prev_hotkey_cb, cx)),
            next_assistant_hotkey_input: cx
                .new(|cx| HotkeyInput::new(next_assistant_hotkey, next_hotkey_cb, cx)),

            visible,
        }
    }
}

impl Render for HotkeysView {
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

        let force_stop_recording = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                // Stop recording if any input is currently recording
                this.assistant_hotkey_input
                    .update(cx, |input, cx| input.stop_recording(cx));
                this.visibility_hotkey_input
                    .update(cx, |input, cx| input.stop_recording(cx));
                this.prev_assistant_hotkey_input
                    .update(cx, |input, cx| input.stop_recording(cx));
                this.next_assistant_hotkey_input
                    .update(cx, |input, cx| input.stop_recording(cx));

                cx.notify();
            }
        });

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
            .on_mouse_down(MouseButton::Left, force_stop_recording)
            .into_any_element()
    }
}
