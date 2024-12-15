use anyhow::Result;
use arboard;
use gpui::{AppContext, Global};

use crate::errors::InputError;

pub struct Clipboard {
    provider: arboard::Clipboard,
}

impl Clipboard {
    pub fn init(cx: &mut AppContext) {
        let clipboard = Clipboard {
            provider: arboard::Clipboard::new().unwrap(),
        };

        cx.set_global(clipboard);
    }

    pub fn get_text(&mut self) -> Result<String> {
        match self.provider.get_text() {
            Ok(text) => Ok(text),
            Err(_) => Err(InputError::ClipboardError.into()),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.provider.set_text(text);
    }
}

impl Global for Clipboard {}
