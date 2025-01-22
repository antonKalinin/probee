use gpui::*;

pub static WIDTH: f32 = 360.;
pub static MIN_HEIGHT: f32 = 40.;
pub static MAX_HEIGHT: f32 = 640.;
pub static MARGIN_TOP: f32 = 56.;
pub static MARGIN_RIGHT: f32 = 16.;

pub struct Window {
    bounds: Bounds<Pixels>,
    display_id: DisplayId,
    hidden: bool,
}

impl Window {
    pub fn init(cx: &mut AppContext) {
        let displays = cx.displays();
        let display = displays.first().unwrap(); // TODO: Support multiple displays

        let size = Size {
            width: px(WIDTH),
            height: px(MIN_HEIGHT),
        };

        // Center origin
        // point(
        //     display.bounds().center().x - size.center().x,
        //     display.bounds().size.height - size.height - px(MARGIN),
        // )

        // Top right origin
        let bounds = Bounds {
            origin: display.bounds().top_right()
                - point(size.width + px(MARGIN_RIGHT), -px(MARGIN_TOP)),
            size,
        };

        let window = Self {
            bounds,
            display_id: display.id(),
            hidden: false,
        };

        cx.set_global::<Self>(window);
    }

    pub fn build_options(&self) -> WindowOptions {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(self.bounds)),
            display_id: Some(self.display_id),
            titlebar: None,
            window_background: WindowBackgroundAppearance::Opaque,
            focus: true,
            show: true,
            kind: WindowKind::PopUp,
            is_movable: true,
            app_id: None,
            window_min_size: None,
            window_decorations: None,
        };

        options
    }

    pub fn show(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|this, cx| {
            cx.activate_window();
            this.hidden = false;
        });
    }

    pub fn hide(cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|this, cx| {
            cx.hide();
            this.hidden = true;
        });
    }

    pub fn set_height(cx: &mut WindowContext, height: f32) {
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

        cx.set_frame(bounds);
    }
}

impl Global for Window {}
