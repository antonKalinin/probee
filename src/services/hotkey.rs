use gpui::{App, Global};
use std::time::{Duration, Instant};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

use crate::errors::*;
use crate::services::{selection, Clipboard};
use crate::state::*;

#[allow(dead_code)]
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
}

impl Global for HotkeyManager {}

impl HotkeyManager {
    pub fn init(cx: &mut App) {
        let manager = GlobalHotKeyManager::new().unwrap();
        let receiver = GlobalHotKeyEvent::receiver().clone();

        let primary_hotkey = HotKey::new(None, Code::MetaLeft);
        let fallback_hotkey = HotKey::new(Some(Modifiers::ALT), Code::Space);
        let visibility_hotkey = HotKey::new(Some(Modifiers::ALT), Code::Tab);

        // On MacOS modifier keys are registered using quartz event services
        // which require additional permissions and if those are not provided
        // registration fails. Fallback hotkey is used in this case.
        manager.register(primary_hotkey).unwrap_or_else(|_err| {
            println!("Failed to register primary hotkey");
            // TODO: Send error report
            manager.register(fallback_hotkey).unwrap();
        });

        manager.register(visibility_hotkey).unwrap();

        cx.set_global::<HotkeyManager>(HotkeyManager { manager });

        cx.spawn(|cx| async move {
            let mut hotkey_ts = Instant::now();

            loop {
                if let Ok(event) = receiver.try_recv() {
                    if event.state == global_hotkey::HotKeyState::Released {
                        let _ = cx.update_global::<HotkeyManager, _>(|_manager, cx| {
                            if !(event.id() == primary_hotkey.id()
                                || event.id() == fallback_hotkey.id()
                                || event.id() == visibility_hotkey.id())
                            {
                                return;
                            }

                            if event.id() == visibility_hotkey.id() {
                                toggle_visible(cx);
                                return;
                            }

                            let key_pressed_at = hotkey_ts;
                            let now = Instant::now();

                            hotkey_ts = now;

                            if (event.id() == primary_hotkey.id())
                                & (now.duration_since(key_pressed_at) > Duration::from_millis(300))
                            {
                                // the meta key was probably pressed independently
                                return;
                            }

                            set_visible(cx, true);

                            // first try to get screen text by selection
                            let input_text = selection::get_text();

                            if let Some(input_text) = input_text.ok() {
                                set_input(cx, input_text);
                                return;
                            }

                            // selection failed, try to get text from clipboard
                            let clipboard = cx.global_mut::<Clipboard>();
                            let input_text = clipboard.get_text();

                            if input_text.is_err() {
                                let err = input_text.unwrap_err();
                                set_error(cx, Some(err));
                                return;
                            }

                            let input_text = input_text.unwrap();

                            if input_text.is_empty() {
                                let err = InputError::EmptyTextInputError.into();
                                set_error(cx, Some(err));

                                return;
                            }

                            set_input(cx, input_text);
                        });
                    }
                }

                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
            }
        })
        .detach();
    }
}
