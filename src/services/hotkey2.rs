use anyhow::Result;
use gpui::{App, Global};

use std::fmt::{self, Display, Formatter};
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use crate::errors::*;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    KeyDown,
    KeyUp,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    event_type: KeyEventType,
    pub keycode: KeyCode,
}

#[derive(Debug)]
pub struct KeyEventsState {
    pub pressed: Vec<KeyCode>,
    pub released: Option<KeyCode>,
    pub released_ts: Option<Instant>,
}

impl KeyEventsState {
    pub fn new() -> Self {
        KeyEventsState {
            pressed: Vec::new(),
            released: None,
            released_ts: None,
        }
    }

    pub fn push(&mut self, key_event: KeyEvent) {
        match key_event.event_type {
            KeyEventType::KeyDown => {
                if !self.pressed.contains(&key_event.keycode) {
                    if self.pressed.len() > 4 {
                        // remove oldest key if we have more than 4 keys pressed
                        self.pressed.remove(0);
                    }

                    self.pressed.push(key_event.keycode);
                }
            }
            KeyEventType::KeyUp => {
                if let Some(index) = self.pressed.iter().position(|&k| k == key_event.keycode) {
                    self.released = Some(self.pressed.remove(index));
                } else {
                    self.released = Some(key_event.keycode);
                }

                self.released_ts = Some(Instant::now());
            }
            KeyEventType::Unknown => {}
        }
    }

    pub fn reset_released(&mut self) {
        self.released = None;
        self.released_ts = None;
    }

    /// Valid hotkey can be either:
    /// 1. A single modifier key pressed twice
    /// 2. A combination of currently pressed modifier keys and a single key
    pub fn into_hotkey(&self) -> Option<HotKey> {
        if self.pressed.is_empty() {
            return None;
        }

        if let Some(released) = self.released {
            if released.is_modifier() && self.pressed.len() == 1 && self.pressed[0] == released {
                // Case: A single modifier key pressed twice
                return Some(HotKey::new(released, vec![], true).ok()?);
            }
        }

        match self.pressed.as_slice() {
            [a, b] => {
                return if a.is_modifier() && !b.is_modifier() {
                    Some(HotKey::new(*b, vec![*a], false).ok()?)
                } else {
                    None
                }
            }
            [a, b, c] => {
                return if a.is_modifier() && b.is_modifier() && !c.is_modifier() {
                    Some(HotKey::new(*c, vec![*a, *b], false).ok()?)
                } else {
                    None
                }
            }
            [a, b, c, d] => {
                return if a.is_modifier() && b.is_modifier() && c.is_modifier() && !d.is_modifier()
                {
                    Some(HotKey::new(*d, vec![*a, *b, *c], false).ok()?)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HotKey {
    key: KeyCode,
    mods: Vec<KeyCode>,
    double_pressed: bool,
}

impl HotKey {
    pub fn new(key: KeyCode, mods: Vec<KeyCode>, double_pressed: bool) -> Result<Self> {
        if double_pressed && !key.is_modifier() {
            return Err(HotkeyError::InvalidHotkeyFormat.into());
        }

        if !double_pressed && mods.is_empty() {
            return Err(HotkeyError::InvalidHotkeyFormat.into());
        }

        if mods.iter().any(|m| !m.is_modifier()) {
            return Err(HotkeyError::InvalidHotkeyFormat.into());
        }

        Ok(HotKey {
            key,
            mods,
            double_pressed,
        })
    }
}

impl Display for HotKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.double_pressed {
            write!(f, "{}+{}", self.key, self.key)
        } else {
            let mods_str = self
                .mods
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join("+");

            write!(f, "{}+{}", mods_str, self.key)
        }
    }
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

pub struct GlobalHotkeyManager {
    key_event_channel: Option<Sender<KeyEvent>>,
    hotkey_channel: Option<Sender<HotKey>>,
}

impl Global for GlobalHotkeyManager {}

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

            let mut state = KeyEventsState::new();

            loop {
                let event = rx.try_recv();

                if event.is_err() {
                    cx.background_executor()
                        .timer(Duration::from_millis(50))
                        .await;
                    continue;
                }

                let event = event.unwrap();

                if state
                    .released_ts
                    .map(|ts| ts.elapsed().as_millis() > 300)
                    .unwrap_or(false)
                {
                    state.reset_released();
                }

                state.push(event);

                let _ = cx.update(|cx| {
                    let manager = cx.global_mut::<GlobalHotkeyManager>();

                    if let Some(key_event_channel) = manager.key_event_channel.as_ref() {
                        let _ = key_event_channel.send(event);
                    }

                    if let Some(hotkey_channel) = manager.hotkey_channel.as_ref() {
                        if let Some(hotkey) = state.into_hotkey() {
                            let _ = hotkey_channel.send(hotkey);
                        }
                    }
                });

                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
            }
        })
        .detach();

        let manager = GlobalHotkeyManager {
            key_event_channel: None,
            hotkey_channel: None,
        };

        cx.set_global(manager);
    }

    pub fn set_key_event_channel(&mut self, channel: Option<Sender<KeyEvent>>) {
        self.key_event_channel = channel;
    }

    pub fn set_hotkey_channel(&mut self, channel: Option<Sender<HotKey>>) {
        self.hotkey_channel = channel;
    }
}
