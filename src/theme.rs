use gpui::*;

fn load_fonts(cx: &mut AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            if let Some(font_bytes) = cx.asset_source().load(&font_path)? {
                embedded_fonts.push(font_bytes);
            }
        }
    }
    cx.text_system().add_fonts(embedded_fonts)
}

#[derive(Debug, Clone, Default)]
pub enum WindowBackgroundAppearanceContent {
    Blurred {
        opacity: f32,
    },
    Transparent {
        opacity: f32,
    },
    #[default]
    Opaque,
}

impl From<WindowBackgroundAppearanceContent> for WindowBackgroundAppearance {
    fn from(content: WindowBackgroundAppearanceContent) -> Self {
        match content {
            WindowBackgroundAppearanceContent::Blurred { .. } => {
                WindowBackgroundAppearance::Blurred
            }
            WindowBackgroundAppearanceContent::Transparent { .. } => {
                WindowBackgroundAppearance::Transparent
            }
            WindowBackgroundAppearanceContent::Opaque => WindowBackgroundAppearance::Opaque,
        }
    }
}

impl WindowBackgroundAppearanceContent {
    pub fn opacity(&self) -> f32 {
        match self {
            WindowBackgroundAppearanceContent::Blurred { opacity }
            | WindowBackgroundAppearanceContent::Transparent { opacity } => *opacity,
            WindowBackgroundAppearanceContent::Opaque => 1.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Theme {
    // typography
    pub font_sans: SharedString,
    pub font_mono: SharedString,
    pub line_height: Rems,
    pub text_size: Pixels,

    // window
    pub window_background: Option<WindowBackgroundAppearanceContent>,

    // colors
    pub background: Hsla,
    pub background_secondary: Hsla,
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_hover: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub border: Hsla,
    pub border_secondary: Hsla,
    pub input: Hsla,
    pub text: Hsla,
    pub text_foreground: Hsla,
    pub subtext: Hsla,

    pub amber400: Hsla,
    pub red500: Hsla,
    pub sky500: Hsla,
    pub sky600: Hsla,
}

impl Theme {
    pub fn init(cx: &mut AppContext) {
        load_fonts(cx).expect("Failed to load fonts");

        // Light theme
        let theme = Theme {
            window_background: Some(WindowBackgroundAppearanceContent::Opaque),

            // colors
            background: Hsla::from(rgb(0xfafaf9)),
            background_secondary: Hsla::from(rgb(0xf5f5f4)),
            text: Hsla::from(rgb(0x0c0a09)),
            text_foreground: Hsla::from(rgb(0xf5f5f4)),
            subtext: Hsla::from(rgb(0x44403c)),
            border: Hsla::from(rgb(0x78716c)),
            border_secondary: Hsla::from(rgb(0xa8a29e)),
            primary: Hsla::from(rgb(0x78716c)),
            primary_hover: Hsla::from(rgb(0x57534e)),
            primary_foreground: Hsla::from(rgb(0xfafaf9)),
            secondary: Hsla::from(rgb(0xe7e5e4)),
            secondary_hover: Hsla::from(rgb(0xd6d3d1)),
            secondary_foreground: Hsla::from(rgb(0xf5f5f4)),
            muted: Hsla::from(rgb(0xf5f5f4)),
            muted_foreground: Hsla::from(rgb(0x0c0a09)),
            destructive: Hsla::from(rgb(0xff0000)),
            destructive_foreground: Hsla::from(rgb(0xf5f5f4)),
            input: Hsla::from(rgb(0xf5f5f4)),

            // tailwind
            amber400: Hsla::from(rgb(0xfbbf24)),
            red500: Hsla::from(rgb(0xef4444)),
            sky500: Hsla::from(rgb(0x0ea5e9)),
            sky600: Hsla::from(rgb(0x0284c7)),

            // typography
            font_sans: "Inter".into(),
            font_mono: "JetBrains Mono".into(),
            line_height: rems(1.25),
            text_size: px(14.),
        };

        cx.set_global(theme);
    }
}

impl Global for Theme {}
