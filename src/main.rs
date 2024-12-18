use dotenv::dotenv;
use gpui::*;

mod assets;
mod errors;
mod events;
mod hotkey;
mod root;
mod services;
mod state;
mod storage;
mod theme;
mod ui;
mod window;

use crate::{
    assets::Assets, hotkey::HotkeyManager, root::Root, services::*, theme::Theme, window::Window,
};

#[async_std::main]
async fn main() {
    dotenv().ok();

    let app = App::new().with_assets(Assets);

    app.run(|cx: &mut AppContext| {
        Assistant::init(cx);
        Clipboard::init(cx);
        Theme::init(cx);
        Window::init(cx);

        let window_options = cx.global::<Window>().build_options();

        let _ = cx.open_window(window_options, |cx| {
            HotkeyManager::init(cx);

            // builing root view and returning it to render
            Root::build(cx)
        });
    });
}
