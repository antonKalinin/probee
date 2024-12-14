use gpui::*;
use std::time::Duration;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

use crate::services::selection;
use crate::state::StateController;
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

                            let selected_text = selection::get_text();

                            if selected_text.is_err() {
                                let err = selected_text.unwrap_err();
                                StateController::update(|this, cx| this.set_error(cx, err), cx);
                                return;
                            }

                            let text = selected_text.unwrap();
                            let empty_text = "".to_string();

                            if text.is_empty() {
                                // TODO: Show error
                                return;
                            }

                            StateController::update(|this, cx| this.set_input(cx, text), cx);
                            StateController::update(|this, cx| this.set_output(cx, empty_text), cx);
                            StateController::update(|this, cx| this.set_loading(cx, true), cx);
                            StateController::update(|this, cx| this.request_assistant(cx), cx);
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
