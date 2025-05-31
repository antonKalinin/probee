use gpui::prelude::FluentBuilder;
use gpui::*;

use std::sync::mpsc;
use std::time::Duration;

use crate::services::{GlobalHotkeyManager, KeyEvent};
use crate::state::settings_state::*;
use crate::ui::Theme;

pub struct HotkeyInput {
    recording: bool,
    keystroke: Option<String>,
}

impl HotkeyInput {
    pub fn new(
        keystroke: Option<String>,
        _state: &Entity<SettingsState>,
        _cx: &mut Context<Self>,
    ) -> Self {
        HotkeyInput {
            keystroke,
            recording: false,
        }
    }

    pub fn record_global_key_events(&mut self, cx: &mut Context<Self>) {
        self.recording = true;

        cx.spawn(
            async |weak_entity: WeakEntity<HotkeyInput>, cx: &mut AsyncApp| {
                let (tx, rx) = mpsc::channel::<KeyEvent>();

                let _ = cx.update_global(|hotkey_manager: &mut GlobalHotkeyManager, _cx| {
                    hotkey_manager.set_key_event_channel(Some(tx));
                });

                loop {
                    let event = rx.try_recv();

                    if let Ok(key_event) = event {
                        println!("Key event: {:?}", key_event);

                        if let Some(weak_self) = weak_entity.upgrade() {
                            let _ = weak_self.update(cx, |this, cx| {
                                this.recording = false;

                                cx.notify();
                                cx.global_mut::<GlobalHotkeyManager>()
                                    .set_key_event_channel(None);
                            });
                        }
                        break;
                    }

                    cx.background_executor()
                        .timer(Duration::from_millis(50))
                        .await;
                }
            },
        )
        .detach();

        cx.notify();
    }
}

impl Render for HotkeyInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| this.record_global_key_events(cx)
        });

        let display_text = if let Some(keystroke) = &self.keystroke {
            keystroke.clone()
        } else {
            "Record Hotkey".to_string()
        };

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
            .when(self.recording, |this| this.child("Recording..."))
            .when(!self.recording, |this| this.child(display_text))
    }
}
