use gpui::*;

fn load_fonts(cx: &mut App) -> gpui::Result<()> {
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
    pub heading_size: Pixels,
    pub text_size: Pixels,
    pub subtext_size: Pixels,

    // window
    pub window_background: Option<WindowBackgroundAppearanceContent>,

    // colors
    pub background: Hsla,
    pub foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub warning: Hsla,
    pub border: Hsla,
    pub input: Hsla,
}

/**

Light theme

--background: 0 0% 100%;
--foreground: 0 0% 3.9%;
--primary: 0 0% 9%;
--primary-foreground: 0 0% 98%;
--secondary: 0 0% 96.1%;
--secondary-foreground: 0 0% 9%;
--muted: 0 0% 96.1%;
--muted-foreground: 0 0% 45.1%;
--accent: 0 0% 96.1%;
--accent-foreground: 0 0% 9%;
--destructive: 0 84.2% 60.2%;
--destructive-foreground: 0 0% 98%;
--border: 0 0% 89.8%;
--input: 0 0% 89.8%;

*/

impl Theme {
    fn ligth_theme() -> Self {
        Theme {
            window_background: Some(WindowBackgroundAppearanceContent::Opaque),

            // colors
            background: hsla(0., 0., 1., 1.),
            foreground: hsla(0., 0., 0.039, 1.),
            primary: hsla(0., 0., 0.09, 1.),
            primary_foreground: hsla(0., 0., 0.98, 1.),
            secondary: hsla(0., 0., 0.961, 1.),
            secondary_foreground: hsla(0., 0., 0.09, 1.),
            muted: hsla(0., 0., 0.961, 1.),
            muted_foreground: hsla(0., 0., 0.451, 1.),
            accent: hsla(0., 0., 0.961, 1.),
            accent_foreground: hsla(0., 0., 0.09, 1.),
            destructive: hsla(0., 0.842, 0.602, 1.),
            destructive_foreground: hsla(0., 0., 0.98, 1.),
            warning: Hsla::from(rgb(0xfbbf24)),
            border: hsla(0., 0., 0.898, 1.),
            input: hsla(0., 0., 0.898, 1.),

            // typography
            font_sans: "Inter".into(),
            font_mono: "JetBrains Mono".into(),
            line_height: rems(1.25),
            heading_size: px(18.),
            text_size: px(14.),
            subtext_size: px(12.),
        }
    }

    pub fn init(cx: &mut App) {
        load_fonts(cx).expect("Failed to load fonts");

        let theme = Theme::ligth_theme();

        cx.set_global(theme);
    }
}

impl Global for Theme {}
