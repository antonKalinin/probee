use gpui::prelude::FluentBuilder;
use gpui::{
    div, AsyncApp, Context, FontWeight, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, SharedString, Styled, WeakEntity, Window,
};
use uuid::Uuid;

use std::sync::mpsc;
use std::time::Duration;

use crate::services::{GlobalHotkeyManager, HotKey, KeyEvent};
use crate::ui::Theme;

pub struct HotkeyInput {
    hotkey: Option<HotKey>,
    recording_id: Option<Uuid>,
    recording_text: SharedString,
    on_hotkey_set: Box<dyn Fn(HotKey, &mut Context<Self>)>,
}

impl HotkeyInput {
    pub fn new(
        hotkey: Option<HotKey>,
        on_hotkey_set: Box<dyn Fn(HotKey, &mut Context<Self>) + 'static>,
        _cx: &mut Context<Self>,
    ) -> Self {
        HotkeyInput {
            hotkey,
            recording_id: None,
            recording_text: SharedString::new("Recording...".to_string()),
            on_hotkey_set,
        }
    }

    pub fn record_global_key_events(&mut self, cx: &mut Context<Self>) {
        let recodring_id = Uuid::new_v4();
        self.recording_id = Some(recodring_id);
        self.recording_text = SharedString::new("Recording...".to_string());

        cx.spawn(
            async move |weak_entity: WeakEntity<HotkeyInput>, cx: &mut AsyncApp| {
                let (tx_key, rx_key) = mpsc::channel::<KeyEvent>();
                let (tx_hotkey, rx_hotkey) = mpsc::channel::<HotKey>();

                let _ = cx.update_global(|hotkey_manager: &mut GlobalHotkeyManager, _cx| {
                    hotkey_manager.set_key_event_channel(Some(tx_key));
                    hotkey_manager.set_hotkey_channel(Some(tx_hotkey));
                });

                if let Some(this) = weak_entity.upgrade() {
                    loop {
                        let stop_recording = this.update::<bool, AsyncApp>(cx, |this, _cx| {
                            this.recording_id
                                .as_ref()
                                .map(|id| *id != recodring_id)
                                .unwrap_or(true)
                        });

                        if stop_recording.unwrap_or(true) {
                            let _ = cx.update_global(
                                |hotkey_manager: &mut GlobalHotkeyManager, _cx| {
                                    hotkey_manager.set_key_event_channel(None);
                                    hotkey_manager.set_hotkey_channel(None);
                                },
                            );
                            break;
                        }

                        let key = rx_key.try_recv();
                        let hotkey = rx_hotkey.try_recv();

                        if let Ok(hotkey) = hotkey {
                            this.update(cx, |this, cx| {
                                this.hotkey = Some(hotkey.clone());
                                this.recording_id = None;

                                (this.on_hotkey_set)(hotkey, cx);

                                cx.notify();
                            })
                            .ok();
                        }

                        if let Ok(key_event) = key {
                            this.update(cx, |this, cx| {
                                this.recording_text =
                                    SharedString::new(key_event.keycode.to_string());

                                cx.notify();
                            })
                            .ok();
                        }

                        cx.background_executor()
                            .timer(Duration::from_millis(50))
                            .await;
                    }
                }
            },
        )
        .detach();

        cx.notify();
    }

    pub fn stop_recording(&mut self, cx: &mut Context<Self>) {
        self.recording_id = None;
        self.recording_text = self
            .hotkey
            .as_ref()
            .map(|hotkey| SharedString::new(hotkey.to_string()))
            .unwrap_or_else(|| SharedString::new("Record Hotkey".to_string()));

        cx.notify();
    }
}

impl Render for HotkeyInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                cx.stop_propagation();
                this.record_global_key_events(cx);
            }
        });

        let display_text = if let Some(hotkey) = &self.hotkey {
            hotkey.to_string()
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
            .when(self.recording_id.is_some(), |this| {
                this.child(self.recording_text.clone())
                    .bg(theme.primary.alpha(0.1))
            })
            .when(self.recording_id.is_none(), |this| this.child(display_text))
    }
}
