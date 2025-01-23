use std::env::set_var;

use dotenv::dotenv;
use gpui::*;

mod assets;
mod errors;
mod events;
mod hotkey;
mod root;
mod services;
mod state;
mod theme;
mod ui;
mod window;

use crate::{
    assets::Assets, hotkey::HotkeyManager, root::Root, services::*, state::*, theme::Theme,
    window::Window,
};

#[async_std::main]
async fn main() {
    dotenv().ok();

    let app = App::new().with_assets(Assets);

    app.run(|cx: &mut AppContext| {
        Api::init(cx);
        Assistant::init(cx);
        Auth::init(cx);
        Clipboard::init(cx);
        StateController::init(cx);
        Storage::init(cx);
        Theme::init(cx);
        Window::init(cx);

        let window_options = cx.global::<Window>().build_options();

        let _ = cx.open_window(window_options, |cx| {
            HotkeyManager::init(cx);

            // reading storage and initalizing state
            let storage = cx.global::<Storage>();
            let access_token = storage.get("access_token");

            if let Some(_) = access_token {
                set_authenticated(cx, true);
            }

            // builing root view and returning it to render
            Root::build(cx)
        });
    });
}
