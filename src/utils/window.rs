use gpui::{
    point, px, App, Bounds, Pixels, Size, WindowBackgroundAppearance, WindowBounds, WindowKind,
    WindowOptions,
};

pub static WIDTH: f32 = 360.;
pub static MIN_HEIGHT: f32 = 40.;
pub static MAX_HEIGHT: f32 = 640.;
pub static MARGIN_TOP: f32 = 56.;
pub static MARGIN_RIGHT: f32 = 16.;

pub fn window_options(cx: &mut App) -> WindowOptions {
    let displays = cx.displays();
    let display = displays.first().unwrap(); // TODO: Support multiple displays

    let size = Size {
        width: px(WIDTH),
        height: px(MIN_HEIGHT),
    };

    // Top right origin
    let bounds = Bounds {
        origin: display.bounds().top_right()
            - point(size.width + px(MARGIN_RIGHT), -px(MARGIN_TOP)),
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

pub fn window_bounds(cx: &mut App, height: f32) -> Bounds<Pixels> {
    let displays = cx.displays();
    let display = displays.first().unwrap();

    let height = height.max(MIN_HEIGHT);
    let height = height.min(MAX_HEIGHT);

    let size = Size {
        width: px(WIDTH),
        height: px(height),
    };

    let bounds = Bounds {
        origin: display.bounds().top_right()
            - point(
                size.width + px(MARGIN_RIGHT),
                -(size.height + px(MARGIN_TOP)),
            ),
        size,
    };

    bounds
}
