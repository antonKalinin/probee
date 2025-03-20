use dotenv::dotenv;
use gpui::{App, Application};
use std::panic;

mod assets;
mod errors;
mod events;
mod root;
mod services;
mod state;
mod theme;
mod ui;
mod utils;

use crate::assets::Assets;
use crate::root::Root;
use crate::services::*;
use crate::state::GlobalState;
use crate::theme::Theme;
use crate::utils::devtools;

#[async_std::main]
async fn main() {
    panic::set_hook(Box::new(|panic_info| {
        devtools::panic_gracefully(panic_info)
    }));

    dotenv().ok();

    let app = Application::new().with_assets(Assets);

    app.run(|cx: &mut App| {
        Api::init(cx);
        Assistant::init(cx);
        Auth::init(cx);
        Clipboard::init(cx);
        GlobalState::init(cx);
        HotkeyManager::init(cx);
        Storage::init(cx);
        Theme::init(cx);

        let window_options = utils::window_options(cx);

        let _ = cx.open_window(window_options, |window, cx| {
            // builing root view and returning it to render
            Root::build(cx, window)
        });
    });
}
