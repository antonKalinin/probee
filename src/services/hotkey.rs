use gpui::{App, Global};
use std::time::{Duration, Instant};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

use crate::errors::*;
use crate::services::selection;
use crate::state::app_state::*;

#[allow(dead_code)]
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
}

impl Global for HotkeyManager {}

impl HotkeyManager {
    pub fn init(cx: &mut App) {
        let manager = GlobalHotKeyManager::new().unwrap();
        let receiver = GlobalHotKeyEvent::receiver().clone();

        // Note: On MacOS modifier keys are registered using quartz event services
        // which require additional permissions and if those are not provided
        // registration fails. Keep this in mind in case of registering somthing like
        // Cmd+Cmd or Alt+Alt.
        let hotkey = HotKey::new(Some(Modifiers::ALT), Code::Tab);

        manager.register(hotkey).unwrap();
        cx.set_global::<HotkeyManager>(HotkeyManager { manager });

        cx.spawn(async move |cx| {
            let mut hotkey_pressed_at: Option<Instant> = None;
            let long_press_duration = Duration::from_millis(300);

            // Dependsing on press duration, either run the action or toggle visibility
            // Short press: toggle visibility
            // Long press: show and run currently selected assistant
            loop {
                // Hotkey is pressed and not released yet, check if it's long press
                // If it's long press, show and run currently selected assistant
                if let Some(pressed_at) = hotkey_pressed_at {
                    if pressed_at.elapsed() > long_press_duration {
                        // reset times to prevent multiple triggers
                        hotkey_pressed_at = None;

                        let _ = cx.update_global::<HotkeyManager, _>(|_, cx| {
                            set_visible(cx, true);
                            HotkeyManager::set_assistant_input(cx);
                        });
                    }
                }

                if let Ok(event) = receiver.try_recv() {
                    let hotkey_event = event.id() == hotkey.id();

                    if event.state == global_hotkey::HotKeyState::Pressed && hotkey_event {
                        hotkey_pressed_at = Some(Instant::now());
                    }

                    if event.state == global_hotkey::HotKeyState::Released && hotkey_event {
                        // Short press bacause hotkey was released before long press duration
                        if hotkey_pressed_at.is_some() {
                            hotkey_pressed_at = None;

                            let _ = cx.update_global::<HotkeyManager, _>(|_, cx| {
                                toggle_visible(cx);
                            });
                        }
                    }
                }

                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
            }
        })
        .detach();
    }

    fn set_assistant_input(cx: &mut App) {
        let input_text = selection::get_text();

        match input_text {
            Ok(text) => {
                if text.is_empty() {
                    set_error(cx, Some(InputError::EmptyTextInputError.into()));
                } else {
                    set_input(cx, text);
                }
            }
            Err(err) => {
                set_error(cx, Some(err));
            }
        }
    }
}
