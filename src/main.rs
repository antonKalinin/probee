use dotenv::dotenv;
use events::AppEvent;
use gpui::{App, Application};
use std::panic;

mod app;
mod assets;
mod errors;
mod events;
mod services;
mod settings;
mod state;
mod theme;
mod ui;
mod utils;

use crate::app::AppRoot;
use crate::assets::Assets;
use crate::services::*;
use crate::settings::SettingsRoot;
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

        let app_window_options = utils::app_window_options(cx);
        let app_window = cx.open_window(app_window_options, AppRoot::build);
        let app_entity = app_window.unwrap().entity(cx).unwrap();

        let _ = cx
            .subscribe(&app_entity, |_app_root, event, cx| match event {
                AppEvent::OpenSettings => {
                    let settings_window_options = utils::settings_window_options(cx);
                    let _ = cx.open_window(settings_window_options, SettingsRoot::build);
                    cx.activate(true);
                }
                _ => {}
            })
            .detach();
    });
}
