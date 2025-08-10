use gpui::{
    point, px, App, AppContext, BorrowAppContext, Bounds, Entity, Global, Size, TitlebarOptions,
    WindowBackgroundAppearance, WindowBounds, WindowHandle, WindowKind, WindowOptions,
};

use crate::app::AppRoot;
use crate::settings::SettingsRoot;
use crate::ui::Root;

pub static APP_WIDTH: f32 = 360.;
pub static APP_MIN_HEIGHT: f32 = 40.;
pub static APP_MARGIN_TOP: f32 = 44.;
pub static APP_MARGIN_RIGHT: f32 = 16.;

pub static SETTINGS_WIDTH: f32 = 600.;
pub static SETTINGS_HEIGHT: f32 = 400.;

pub fn app_window_options(cx: &mut App) -> WindowOptions {
    let displays = cx.displays();

    if displays.is_empty() {
        panic!("No displays found. Cannot create app window.");
    }

    let display = displays.first().unwrap();

    let size = Size {
        width: px(APP_WIDTH),
        height: px(APP_MIN_HEIGHT),
    };

    // Top right origin
    let bounds = Bounds {
        origin: display.bounds().top_right()
            - point(size.width + px(APP_MARGIN_RIGHT), -px(APP_MARGIN_TOP)),
        size,
    };

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        display_id: Some(display.id()),
        titlebar: None,
        window_background: WindowBackgroundAppearance::Opaque,
        focus: true,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        app_id: None,
        window_min_size: None,
        window_decorations: None,
    };

    options
}

pub fn settings_window_options(cx: &mut App) -> WindowOptions {
    let displays = cx.displays();
    let display = displays.first().unwrap();

    let size = Size {
        width: px(SETTINGS_WIDTH),
        height: px(SETTINGS_HEIGHT),
    };

    let bounds = Bounds {
        origin: point(
            display.bounds().center().x - size.center().x,
            display.bounds().center().y - size.center().y * 2.,
        ),
        size,
    };

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        display_id: Some(display.id()),
        titlebar: Some(TitlebarOptions {
            appears_transparent: true,
            ..Default::default()
        }),
        window_background: WindowBackgroundAppearance::Opaque,
        focus: false,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        app_id: None,
        window_min_size: Some(size),
        window_decorations: None,
    };

    options
}

pub struct WindowsState {
    pub app_window_handle: Option<WindowHandle<AppRoot>>,
    pub settings_window_handle: Option<WindowHandle<Root>>,
}

#[derive(Clone)]
pub struct Windows {
    state: Entity<WindowsState>,
}

impl Windows {
    pub fn init(cx: &mut App) {
        let state = cx.new(|_cx| WindowsState {
            app_window_handle: None,
            settings_window_handle: None,
        });

        let windows = Windows { state };

        cx.set_global(windows);
    }

    pub fn open_app(cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            this.state.update(cx, |state, cx| {
                if state.app_window_handle.is_none() {
                    let app_window_options = app_window_options(cx);
                    let app_window = cx.open_window(app_window_options, AppRoot::build).unwrap();
                    state.app_window_handle = Some(app_window);
                }
            });
        });
    }

    pub fn close_app(cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            this.state.update(cx, |state, cx| {
                if let Some(handle) = state.app_window_handle {
                    let _ = handle.update(cx, |_view, window, _cx| {
                        window.remove_window();
                    });

                    state.app_window_handle = None;
                }
            });
        });
    }

    pub fn toggle_app(cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            this.state.update(cx, |state, cx| {
                if let Some(handle) = state.app_window_handle {
                    let _ = handle.update(cx, |_view, window, _cx| {
                        window.remove_window();
                    });

                    state.app_window_handle = None;
                } else {
                    let app_window_options = app_window_options(cx);
                    let app_window = cx.open_window(app_window_options, AppRoot::build).unwrap();

                    state.app_window_handle = Some(app_window);
                }
            });
        });
    }

    pub fn open_settings(cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            this.state.update(cx, |state, cx| {
                if let Some(handle) = state.settings_window_handle {
                    let _ = handle.update(cx, |_view, window, _cx| {
                        window.remove_window();
                    });
                }

                let settings_window_options = settings_window_options(cx);
                let settings_window = cx
                    .open_window(settings_window_options, SettingsRoot::build)
                    .unwrap();

                state.settings_window_handle = Some(settings_window);

                cx.activate(true);

                let _ = settings_window.update(cx, |_, window, cx| {
                    window.on_window_should_close(cx, |_window, cx| {
                        cx.update_global::<Self, _>(|this, cx| {
                            this.state.update(cx, |state, _cx| {
                                state.settings_window_handle = None;
                            });
                        });

                        true
                    });
                });
            });
        });
    }
}

impl Global for Windows {}
