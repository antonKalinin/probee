use accessibility::{AXAttribute, AXUIElement};
use accessibility_sys::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use anyhow::{bail, Result};
use core_foundation::string::CFString;

const APPLE_SCRIPT: &str = r#"
use AppleScript version "2.4"
use scripting additions
use framework "Foundation"
use framework "AppKit"

set savedAlertVolume to alert volume of (get volume settings)

-- Back up clipboard contents:
set savedClipboard to the clipboard

set thePasteboard to current application's NSPasteboard's generalPasteboard()
set theCount to thePasteboard's changeCount()

tell application "System Events"
    set volume alert volume 0
end tell

-- Copy selected text to clipboard:
tell application "System Events" to keystroke "c" using {command down}
delay 0.1 -- Without this, the clipboard may have stale data.

tell application "System Events"
    set volume alert volume savedAlertVolume
end tell

if thePasteboard's changeCount() is theCount then
    return ""
end if

set theSelectedText to the clipboard

set the clipboard to savedClipboard

theSelectedText
"#;

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
        bail!("No selected element");
    };

    let Some(selected_text) = selected_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXSelectedTextAttribute,
        )))
        .map(|text| text.downcast_into::<CFString>())
        .ok()
        .flatten()
    else {
        bail!("No selected text")
    };

    Ok(selected_text.to_string())
}

/**
 * Get the selected text using the clipboard and AppleScript
 */
fn get_selected_text_fallback() -> Result<String> {
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(APPLE_SCRIPT)
        .output()?;
    if output.status.success() {
        let content = String::from_utf8(output.stdout)?;
        let content = content.trim();
        Ok(content.to_owned())
    } else {
        let err: String = output
            .stderr
            .into_iter()
            .map(|c| c as char)
            .collect::<String>()
            .into();

        bail!(err)
    }
}
