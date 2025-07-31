use gpui::{point, px, App, Bounds, Pixels, Point, Size};

pub static APP_WIDTH: f32 = 360.;
pub static APP_MIN_HEIGHT: f32 = 40.;
pub static APP_MAX_HEIGHT: f32 = 640.;
pub static APP_MARGIN_TOP: f32 = 44.;
pub static APP_MARGIN_RIGHT: f32 = 16.;

pub static SETTINGS_WIDTH: f32 = 600.;
pub static SETTINGS_MIN_HEIGHT: f32 = 280.;
pub static SETTINGS_MAX_HEIGHT: f32 = 640.;

pub fn app_window_bounds(cx: &mut App, height: f32) -> Bounds<Pixels> {
    let displays = cx.displays();

    let height = height.max(APP_MIN_HEIGHT);
    let height = height.min(APP_MAX_HEIGHT);

    let size = Size {
        width: px(APP_WIDTH),
        height: px(height),
    };

    // For some reason, the displays can be empty at this point.
    if displays.is_empty() {
        return Bounds {
            origin: point(px(0.), px(0.)),
            size,
        };
    }

    let display = displays.first().unwrap();

    Bounds {
        origin: display.bounds().top_right()
            - point(
                size.width + px(APP_MARGIN_RIGHT),
                -(size.height + px(APP_MARGIN_TOP)),
            ),
        size,
    }
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
