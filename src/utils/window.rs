use gpui::{
    point, px, App, Bounds, Pixels, Point, SharedString, Size, TitlebarOptions,
    WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
};

pub static APP_WIDTH: f32 = 360.;
pub static APP_MIN_HEIGHT: f32 = 40.;
pub static APP_MAX_HEIGHT: f32 = 640.;
pub static APP_MARGIN_TOP: f32 = 44.;
pub static APP_MARGIN_RIGHT: f32 = 16.;

pub static SETTINGS_WIDTH: f32 = 600.;
pub static SETTINGS_HEIGHT: f32 = 400.;
pub static SETTINGS_MIN_HEIGHT: f32 = 280.;
pub static SETTINGS_MAX_HEIGHT: f32 = 640.;

pub static PROMPT_WIDTH: f32 = 600.;
pub static PROMPT_HEIGHT: f32 = 560.;

pub fn app_window_options(cx: &mut App) -> WindowOptions {
    let displays = cx.displays();
    let display = displays.first().unwrap(); // TODO: Support multiple displays

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

pub fn app_window_bounds(cx: &mut App, height: f32) -> Bounds<Pixels> {
    let displays = cx.displays();
    let display = displays.first().unwrap();

    let height = height.max(APP_MIN_HEIGHT);
    let height = height.min(APP_MAX_HEIGHT);

    let size = Size {
        width: px(APP_WIDTH),
        height: px(height),
    };

    Bounds {
        origin: display.bounds().top_right()
            - point(
                size.width + px(APP_MARGIN_RIGHT),
                -(size.height + px(APP_MARGIN_TOP)),
            ),
        size,
    }
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
            display.bounds().center().x - size.center().x * 2.,
            display.bounds().center().y - size.center().y * 1.5,
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

pub fn settings_window_bounds(_cx: &mut App, origin: Point<Pixels>, height: f32) -> Bounds<Pixels> {
    let height = height.max(SETTINGS_MIN_HEIGHT);
    let height = height.min(SETTINGS_MAX_HEIGHT);

    let size = Size {
        width: px(SETTINGS_WIDTH),
        height: px(height),
    };

    Bounds {
        origin: point(origin.x, origin.y + px(height)),
        size,
    }
}

pub fn prompt_window_options(cx: &mut App) -> WindowOptions {
    let displays = cx.displays();
    let display = displays.first().unwrap();

    let size = Size {
        width: px(PROMPT_WIDTH),
        height: px(PROMPT_HEIGHT),
    };

    let bounds = Bounds {
        origin: point(
            display.bounds().center().x + px(20.),
            display.bounds().center().y - size.center().y - px(20.),
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
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        app_id: None,
        window_min_size: Some(size),
        window_decorations: None,
    };

    options
}
