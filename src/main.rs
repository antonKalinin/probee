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

use crate::assets::Assets;
use crate::hotkey::HotkeyManager;
use crate::root::Root;
use crate::services::*;
use crate::state::*;
use crate::theme::Theme;
use crate::window::Window;

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
            // hotkey manager requires window context
            HotkeyManager::init(cx);

            // builing root view and returning it to render
            Root::build(cx)
        });
    });
}
