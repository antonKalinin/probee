use anyhow::Result;
use gpui::App;

use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::mpsc;
use std::time::Duration;

use crate::errors::*;
use crate::services::{Storage, StorageKey};
use crate::state::app_state::set_error_async;
use crate::utils::keyboard::KeyCode;

// Core Graphics and Core Foundation bindings
#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGEventTapCreate(
        tap: c_int,
        place: c_int,
        options: c_int,
        events_of_interest: u64,
        callback: extern "C" fn(*mut c_void, c_int, *mut c_void, *mut c_void) -> *mut c_void,
        user_info: *mut c_void,
    ) -> *mut c_void;

    fn CGEventTapEnable(tap: *mut c_void, enable: bool);
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: *mut c_void);
    fn CFMachPortCreateRunLoopSource(
        allocator: *mut c_void,
        port: *mut c_void,
        order: c_int,
    ) -> *mut c_void;
    fn CFRunLoopRun();
    fn CGEventGetIntegerValueField(event: *mut c_void, field: c_int) -> u64;
    fn CGEventGetFlags(event: *mut c_void) -> u64;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    static kCFRunLoopCommonModes: *mut c_void;
}

// Constants
const K_CG_SESSION_EVENT_TAP: c_int = 0;
const K_CG_HEAD_INSERT_EVENT_TAP: c_int = 0;
const K_CG_EVENT_TAP_OPTION_LISTEN_ONLY: c_int = 1;
const K_CG_EVENT_KEY_DOWN: u64 = 1 << 10;
const K_CG_EVENT_KEY_UP: u64 = 1 << 11;
const K_CG_EVENT_FLAGS_CHANGED: u64 = 1 << 12;
const K_CG_EVENT_FIELD_KEYBOARD_EVENT_KEYCODE: c_int = 9;

// Modifier flags
const K_CG_EVENT_FLAG_MASK_SHIFT: u64 = 0x00020000;
const K_CG_EVENT_FLAG_MASK_CONTROL: u64 = 0x00040000;
const K_CG_EVENT_FLAG_MASK_ALTERNATE: u64 = 0x00080000;
const K_CG_EVENT_FLAG_MASK_COMMAND: u64 = 0x00100000;

#[derive(Debug)]
pub struct KeyEventsState {
    pub prev_released: KeyCode,
    pub curr_pressed: Vec<KeyCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HotKey {
    pub mods: Vec<KeyCode>,
    pub key: Option<KeyCode>,
}

impl HotKey {
    pub fn new(mods: Vec<KeyCode>, key: Option<KeyCode>) -> Result<Self> {
        if mods.is_empty() {
            return Err(HotkeyError::InvalidHotkeyFormat.into());
        }

        Ok(HotKey { mods, key })
    }

    // TODO: Make sure function accepts either array of 2, 3 or 4 events
    pub fn from_event_sequence(events: Vec<KeyEvent>) -> Option<Self> {
        match events.as_slice() {
            [first, second] => {
                // Case: Modifier Down, Key Down
                if first.keycode.is_modifier()
                    && !second.keycode.is_modifier()
                    && first.event_type == KeyEventType::KeyDown
                    && second.event_type == KeyEventType::KeyDown
                {
                    return Some(HotKey {
                        mods: vec![first.keycode],
                        key: Some(second.keycode),
                    });
                }

                None
            }
            [first, second, third] => {
                // Case 1: Modifier Down, Modifier Up, Modifier Down
                let same_modifier = first.keycode.is_modifier()
                    && first.keycode == second.keycode
                    && second.keycode == third.keycode;

                let double_press = match (first.event_type, second.event_type, third.event_type) {
                    (KeyEventType::KeyDown, KeyEventType::KeyUp, KeyEventType::KeyDown) => true,
                    _ => false,
                };

                if same_modifier && double_press {
                    return Some(HotKey {
                        mods: vec![first.keycode],
                        key: None,
                    });
                }

                // Case 2: Modifier Down, Modifier Down, Key Down
                if first.event_type == KeyEventType::KeyDown
                    && second.event_type == KeyEventType::KeyDown
                    && first.keycode.is_modifier()
                    && second.keycode.is_modifier()
                    && first.keycode != second.keycode
                    && third.event_type == KeyEventType::KeyDown
                {
                    return Some(HotKey {
                        mods: vec![first.keycode, second.keycode],
                        key: Some(third.keycode),
                    });
                }

                None
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    KeyDown,
    KeyUp,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    event_type: KeyEventType,
    keycode: KeyCode,
}

extern "C" fn event_callback(
    _proxy: *mut c_void,
    event_type: c_int,
    event: *mut c_void,
    user_info: *mut c_void,
) -> *mut c_void {
    unsafe {
        let keycode = CGEventGetIntegerValueField(event, K_CG_EVENT_FIELD_KEYBOARD_EVENT_KEYCODE);
        let keycode = KeyCode::try_from(keycode).unwrap_or(KeyCode::Unknown);
        let flags = CGEventGetFlags(event);

        let mut modifiers: Vec<KeyCode> = Vec::new();
        if flags & K_CG_EVENT_FLAG_MASK_COMMAND != 0 {
            modifiers.push(KeyCode::Command);
        }
        if flags & K_CG_EVENT_FLAG_MASK_ALTERNATE != 0 {
            modifiers.push(KeyCode::Option);
        }
        if flags & K_CG_EVENT_FLAG_MASK_CONTROL != 0 {
            modifiers.push(KeyCode::Control);
        }
        if flags & K_CG_EVENT_FLAG_MASK_SHIFT != 0 {
            modifiers.push(KeyCode::Shift);
        }

        // Convert event type to readable format
        let event_type = match event_type as u32 {
            10 => KeyEventType::KeyDown,
            11 => KeyEventType::KeyUp,
            12 => {
                if modifiers.contains(&keycode) {
                    KeyEventType::KeyDown
                } else {
                    KeyEventType::KeyUp
                }
            }
            _ => KeyEventType::Unknown,
        };

        // Send message to main thread via channel
        if !user_info.is_null() {
            let tx = &*(user_info as *const mpsc::Sender<KeyEvent>);
            let _ = tx.send(KeyEvent {
                event_type,
                keycode,
            });
        }
    }

    // Return the event unmodified to allow normal processing
    event
}

pub enum HotKeyCommand {
    RunAssistant,
    ToggleVisibility,
    NextPrompt,
    PrevPrompt,
}

impl HotKeyCommand {
    // Default keystroke - can be overridden
    fn default_keystroke(&self) -> &'static str {
        match self {
            HotKeyCommand::RunAssistant => "alt+tab",
            HotKeyCommand::ToggleVisibility => "alt+`",
            HotKeyCommand::NextPrompt => "alt-2",
            HotKeyCommand::PrevPrompt => "alt-1",
        }
    }
}

#[allow(dead_code)]
pub struct GlobalHotkeyManager;

impl GlobalHotkeyManager {
    pub fn init(cx: &mut App) {
        cx.spawn(async move |cx| {
            // Create a channel for communication between threads
            let (tx, rx) = mpsc::channel::<KeyEvent>();

            unsafe {
                // Create event tap for key events
                let event_mask = K_CG_EVENT_KEY_DOWN | K_CG_EVENT_KEY_UP | K_CG_EVENT_FLAGS_CHANGED;

                let event_tap = CGEventTapCreate(
                    K_CG_SESSION_EVENT_TAP,
                    K_CG_HEAD_INSERT_EVENT_TAP,
                    K_CG_EVENT_TAP_OPTION_LISTEN_ONLY,
                    event_mask,
                    event_callback,
                    Box::into_raw(Box::new(tx)) as *mut c_void,
                );

                if event_tap.is_null() {
                    set_error_async(cx, Some(HotkeyError::TapEventCreationFailure.into()));
                    return;
                }

                // Create run loop source and add to current run loop
                let run_loop_source = CFMachPortCreateRunLoopSource(ptr::null_mut(), event_tap, 0);
                let run_loop = CFRunLoopGetCurrent();

                CFRunLoopAddSource(run_loop, run_loop_source, kCFRunLoopCommonModes);

                // Enable the event tap
                CGEventTapEnable(event_tap, true);

                // Start the run loop (this will block)
                CFRunLoopRun();
            }

            // A sequence of events to form a hotkey
            // To form valid hotkey we need at least 2 events and at most 4 events
            // 2: Modifier Down, Key Down
            // 3: Modifier Down, Modifier Down, Key Down
            // 3: Modifier Down, Modifier Up, Modifier Down (double press of modifier)
            // 4: Modifier Down, Modifier Down, Modifier Down, Key Down
            let mut events: Vec<KeyEvent> = Vec::with_capacity(4);

            loop {
                let event = rx.try_recv();

                if event.is_err() {
                    // If no event is received, we can continue to the next iteration
                    cx.background_executor()
                        .timer(Duration::from_millis(50))
                        .await;
                    continue;
                }

                let event = event.unwrap();

                if events.len() == 4 {
                    events.remove(0); // remove oldest
                }

                // Do not push duplicate events
                if let Some(last_event) = events.last() {
                    if last_event.to_owned() == event {
                        continue; // Skip duplicate event
                    }
                }
                events.push(event);

                // To find a hotkey at any position of the sequence [a, b, c?, d?] we need to check
                // combinations of events in exacly the following order:
                // 1. [a, b]
                // 2. [a, b, c]
                // 3. [a, b, c, d]
                // 4. [b, c]
                // 5. [b, c, d]
                // 6. [c, d]
                let mut combinations = Vec::new();

                combinations.push(events.iter().take(2).cloned().collect::<Vec<_>>());

                if events.len() == 3 {
                    combinations.push(events.iter().take(3).cloned().collect::<Vec<_>>());
                }

                if events.len() == 4 {
                    combinations.push(events.iter().take(4).cloned().collect::<Vec<_>>());
                    combinations.push(events.iter().skip(1).take(2).cloned().collect::<Vec<_>>());
                    combinations.push(events.iter().skip(1).take(3).cloned().collect::<Vec<_>>());
                    combinations.push(events.iter().skip(2).take(2).cloned().collect::<Vec<_>>());
                }

                // Note: event sequence doesn't work as events come with duplications which should be ignored
                // Also if modifier is released while other is still pressed we should update the sequence
                // to remove the released modifier but keep the pressed one.
                // So we need something like a state machine, which we update with each event and then check
                // either the state is valid hotkey or not.
                // Double press of modifier is special case here and should be handled separately.

                for event_sequence in combinations {
                    if let Some(hotkey) = HotKey::from_event_sequence(event_sequence) {
                        // If we have a valid hotkey, we can break the loop
                        println!("Found hotkey: {:?}", hotkey);
                        // Here you can handle the hotkey, e.g. run an assistant or toggle visibility
                        // For now, just print it
                        break;
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
