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
    assets::Assets, hotkey::HotkeyManager, root::Root, services::*, state::*, theme::Theme,
    window::Window,
};

struct EventBus;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let app = App::new().with_assets(Assets);

    app.run(|cx: &mut AppContext| {
        Api::init(cx);
        Assistant::init(cx);
        Clipboard::init(cx);
        StateController::init(cx);
        Theme::init(cx);
        Window::init(cx);

        let window_options = cx.global::<Window>().build_options();
        let state = cx.global::<StateController>().model.clone();

        let _ = cx.open_window(window_options, |cx| {
            HotkeyManager::init(cx);

            let _ = cx.subscribe(&state, |emitter, event, cx| {
                println!("EVENT RECEIVED: {:?}", event);
            });

            // builing root view and returning it to render
            Root::build(cx, state)
        });
    });
}
