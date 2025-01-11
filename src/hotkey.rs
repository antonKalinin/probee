use gpui::*;
use std::time::Duration;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

use crate::errors::*;
use crate::services::{selection, Clipboard};
use crate::state::*;
use crate::window::Window;

#[allow(dead_code)]
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
}

impl Global for HotkeyManager {}

impl HotkeyManager {
    pub fn init(cx: &mut WindowContext) {
        let manager = GlobalHotKeyManager::new().unwrap();
        let receiver = GlobalHotKeyEvent::receiver().clone();

        let mut mods = Modifiers::empty();
        mods.set(Modifiers::SUPER, true);
        mods.set(Modifiers::SHIFT, true);

        let appearence_hotkey = HotKey::new(Some(mods), Code::KeyI); // CMD + SHIFT + I
        let assistant_hotkey = HotKey::new(Some(Modifiers::SUPER), Code::KeyI); // CMD + I

        manager.register(appearence_hotkey).unwrap();
        manager.register(assistant_hotkey).unwrap();

        cx.set_global::<HotkeyManager>(HotkeyManager { manager });

        cx.spawn(|mut cx| async move {
            loop {
                if let Ok(event) = receiver.try_recv() {
                    if event.state == global_hotkey::HotKeyState::Released {
                        let _ = cx.update_global::<HotkeyManager, _>(|_manager, cx| {
                            if event.id() == appearence_hotkey.id() {
                                Window::toggle(cx);
                                return;
                            }

                            // First try to get screen text by selection
                            let input_text = selection::get_text();

                            if let Some(input_text) = input_text.ok() {
                                set_input(cx, input_text);
                                return;
                            }

                            // If selection failed, try to get text from clipboard
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
