use std::panic;
use std::time::Duration;

use dotenv::dotenv;
use events::AppEvent;
use gpui::{App, Application, AsyncApp};

mod app;
mod assets;
mod errors;
mod events;
mod platform;
mod services;
mod settings;
mod state;
mod ui;
mod utils;

use crate::app::AppRoot;
use crate::assets::Assets;
use crate::services::*;
use crate::settings::SettingsRoot;
use crate::state::app_state::*;
use crate::state::settings_state::*;
use crate::ui::{Components, Theme};
use crate::utils::devtools;

fn open_settings(cx: &mut App) {
    let window_handle = get_settings_window_handle(cx);

    if window_handle.is_some() {
        let _ = window_handle.unwrap().update(cx, |_, window, _cx| {
            window.remove_window();
        });
    }

    let settings_window_options = utils::settings_window_options(cx);
    let handle = cx.open_window(settings_window_options, SettingsRoot::build);

    if let Ok(handle) = handle {
        let _ = handle.update(cx, |_, window, cx| {
            window.on_window_should_close(cx, |_window, cx| {
                set_settings_window_handle(cx, None);
                true
            });
        });
    }

    set_settings_window_handle(cx, handle.ok());
    cx.activate(true);
}

#[async_std::main]
async fn main() {
    panic::set_hook(Box::new(|panic_info| {
        devtools::panic_gracefully(panic_info)
    }));

    dotenv().ok();

    let app = Application::new().with_assets(Assets);

    app.run(|cx: &mut App| {
        // services (order of initialization matters as services placed to global context)
        Storage::init(cx);
        Api::init(cx);
        Assistant::init(cx);
        GlobalHotkeyManager::init(cx);
        Theme::init(cx);
        Components::init(cx);
        // state
        AppStateController::init(cx);
        SettingsStateController::init(cx);

        let app_window_options = utils::app_window_options(cx);
        let app_window = cx.open_window(app_window_options, AppRoot::build);
        let app_entity = app_window.as_ref().unwrap().entity(cx).unwrap();

        let _ = cx
            .subscribe(&app_entity, move |_app_root, event, cx| match event {
                AppEvent::OpenSettings => open_settings(cx),
                _ => {}
            })
            .detach();

        // TODO: Log status menu initialization failure
        let menu_handler = platform::init_status_menu(cx);

        if let Ok(rx) = menu_handler {
            cx.spawn(async move |cx: &mut AsyncApp| loop {
                let action = rx.try_recv();

                match action {
                    Ok(platform::MenuAction::OpenApp) => {
                        cx.update(|cx| set_visible(cx, true)).ok();
                    }
                    Ok(platform::MenuAction::OpenSettings) => {
                        cx.update(open_settings).ok();
                    }
                    _ => {}
                }

                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
            })
            .detach();
        }
    });
}
