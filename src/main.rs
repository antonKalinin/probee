use std::panic;
use std::time::Duration;

use dotenv::dotenv;
use gpui::{App, Application, AsyncApp};

mod actions;
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
mod windows;

use crate::actions::*;
use crate::assets::Assets;
use crate::errors::InputError;
use crate::services::*;
use crate::state::app_state::*;
use crate::state::settings_state::SettingsStateController;
use crate::ui::{Components, Theme};
use crate::utils::devtools;
use crate::windows::Windows;

// fn open_settings(cx: &mut App) {
//     let window_handle = get_settings_window_handle(cx);

//     if window_handle.is_some() {
//         let _ = window_handle.unwrap().update(cx, |_, window, _cx| {
//             window.remove_window();
//         });
//     }

//     let settings_window_options = utils::settings_window_options(cx);
//     let handle = cx.open_window(settings_window_options, SettingsRoot::build);

//     if let Ok(handle) = handle {
//         let _ = handle.update(cx, |_, window, cx| {
//             window.on_window_should_close(cx, |_window, cx| {
//                 set_settings_window_handle(cx, None);
//                 true
//             });
//         });
//     }

//     set_settings_window_handle(cx, handle.ok());
//     cx.activate(true);
// }

#[async_std::main]
async fn main() {
    panic::set_hook(Box::new(|panic_info| {
        devtools::panic_gracefully(panic_info)
    }));

    dotenv().ok();

    let app = Application::new().with_assets(Assets);

    app.run(|cx: &mut App| {
        // Order of initialization matters here: storage and state should be initialized before others
        Storage::init(cx);
        AppStateController::init(cx);
        SettingsStateController::init(cx);

        Api::init(cx);
        Assistant::init(cx);
        GlobalHotkeyManager::init(cx);
        Theme::init(cx);
        Components::init(cx);
        Windows::init(cx);

        Windows::open_app(cx);

        // Global actions bindings

        cx.on_action(|_: &ToggleApp, cx| Windows::toggle_app(cx));
        cx.on_action(|_: &CloseApp, cx| Windows::close_app(cx));
        cx.on_action(|_: &OpenSettings, cx| Windows::open_settings(cx));
        cx.on_action(|_: &RunAssistant, cx| {
            match selection::get_text() {
                Ok(text) => {
                    if text.is_empty() {
                        set_error(cx, Some(InputError::EmptyTextInputError.into()));
                    } else {
                        set_input(cx, text);
                    }
                }
                Err(err) => {
                    set_error(cx, Some(err));
                }
            }

            Windows::open_app(cx);
        });

        // TODO: Log status menu initialization failure
        let menu_handler = platform::init_status_menu(cx);

        if let Ok(rx) = menu_handler {
            cx.spawn(async move |cx: &mut AsyncApp| loop {
                let action = rx.try_recv();

                match action {
                    Ok(platform::MenuAction::OpenApp) => {
                        cx.update(Windows::open_app).ok();
                    }
                    Ok(platform::MenuAction::OpenSettings) => {
                        cx.update(Windows::open_settings).ok();
                    }
                    _ => (),
                }

                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
            })
            .detach();
        }
    });
}
