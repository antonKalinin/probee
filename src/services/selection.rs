use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use accessibility::{AXAttribute, AXUIElement};
use accessibility_sys::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use anyhow::{bail, Result};
use arboard::Clipboard;
use core_foundation::string::CFString;
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use crate::errors::InputError;

pub fn get_text() -> Result<String> {
    return match get_selected_text_by_ax() {
        Ok(text) => Ok(text),
        Err(_) => match get_selected_text_fallback() {
            Ok(text) => Ok(text),
            Err(err) => Err(err),
        },
    };
}

/**
 * Get the selected text using the accessibility API
 */
fn get_selected_text_by_ax() -> Result<String> {
    let system_element = AXUIElement::system_wide();
    let Some(selected_element) = system_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXFocusedUIElementAttribute,
        )))
        .map(|element| element.downcast_into::<AXUIElement>())
        .ok()
        .flatten()
    else {
        bail!(InputError::AccessibilityPermissionsMissing);
    };

    let Some(selected_text) = selected_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXSelectedTextAttribute,
        )))
        .map(|text| text.downcast_into::<CFString>())
        .ok()
        .flatten()
    else {
        bail!(InputError::TextSelectionMissing)
    };

    Ok(selected_text.to_string())
}

/**
 * Get the selected text using the clipboard and simulating Cmd+C
 */
fn get_selected_text_fallback() -> Result<String> {
    simulate_cmd_c_with_applescript()?;

    // Wait for clipboard update
    sleep(Duration::from_millis(100));

    // Read copied text
    let mut clipboard = Clipboard::new()?;
    let selected_text = clipboard.get_text()?.trim().to_string();

    if selected_text.is_empty() {
        bail!(InputError::TextSelectionMissing)
    } else {
        Ok(selected_text)
    }
}

fn _simulate_cmd_c_with_cgevent() {
    let keycode: CGKeyCode = 8; // C key
    let event_source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .expect("Could not create CGEventSource.");

    // simulate Cmd+C
    let key_down = CGEvent::new_keyboard_event(event_source.clone(), keycode, true)
        .expect("Could not create CGEvent for Cmd+C key press.");
    key_down.set_flags(CGEventFlags::CGEventFlagCommand);
    key_down.post(CGEventTapLocation::HID);

    sleep(Duration::from_millis(50));

    let key_up = CGEvent::new_keyboard_event(event_source.clone(), keycode, false)
        .expect("Could not create CGEvent for Cmd+C key press.");
    key_up.set_flags(CGEventFlags::CGEventFlagCommand);
    key_up.post(CGEventTapLocation::HID);
}

fn simulate_cmd_c_with_applescript() -> Result<()> {
    const APPLE_SCRIPT: &str =
        r#"tell application "System Events" to keystroke "c" using command down"#;
    let output = Command::new("osascript")
        .arg("-e")
        .arg(APPLE_SCRIPT)
        .output();

    match output {
        Ok(output) => {
            println!("Output status: {:?}", output.status);
            if !output.status.success() {
                bail!(InputError::AccessibilityPermissionsMissing);
            }
        }
        Err(_) => {
            bail!(InputError::AccessibilityPermissionsMissing);
        }
    }

    Ok(())
}
