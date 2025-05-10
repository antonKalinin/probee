use gpui::{
    hsla, point, px, rems, rgb, App, BoxShadow, Global, Hsla, Pixels, Rems, SharedString, Window,
    WindowAppearance,
};
use std::ops::{Deref, DerefMut};

use super::{hsl, Colorize};
use crate::storage::*;

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

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    #[inline]
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

/// Make a BoxShadow like CSS
///
/// e.g:
///
/// If CSS is `box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.1);`
///
/// Then the equivalent in Rust is `box_shadow(0., 0., 10., 0., hsla(0., 0., 0., 0.1))`
#[inline]
pub fn box_shadow(
    x: impl Into<Pixels>,
    y: impl Into<Pixels>,
    blur: impl Into<Pixels>,
    spread: impl Into<Pixels>,
    color: Hsla,
) -> BoxShadow {
    BoxShadow {
        offset: point(x.into(), y.into()),
        blur_radius: blur.into(),
        spread_radius: spread.into(),
        color,
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ThemeColor {
    /// Used for accents such as hover background on MenuItem, ListItem, etc.
    pub accent: Hsla,
    /// Used for accent text color.
    pub accent_foreground: Hsla,
    /// Default background color.
    pub background: Hsla,
    /// Default border color
    pub border: Hsla,
    /// Input caret color (Blinking cursor).
    pub caret: Hsla,
    /// Danger background color.
    pub danger: Hsla,
    /// Danger active background color.
    pub danger_active: Hsla,
    /// Danger text color.
    pub danger_foreground: Hsla,
    /// Danger hover background color.
    pub danger_hover: Hsla,
    /// Default text color.
    pub foreground: Hsla,
    /// Info background color.
    pub info: Hsla,
    /// Info active background color.
    pub info_active: Hsla,
    /// Info text color.
    pub info_foreground: Hsla,
    /// Info hover background color.
    pub info_hover: Hsla,
    /// Border color for inputs such as Input, Dropdown, etc.
    pub input: Hsla,
    /// Link text color.
    pub link: Hsla,
    /// Active link text color.
    pub link_active: Hsla,
    /// Hover link text color.
    pub link_hover: Hsla,
    /// Muted backgrounds such as Skeleton and Switch.
    pub muted: Hsla,
    /// Muted text color, as used in disabled text.
    pub muted_foreground: Hsla,
    /// Primary background color.
    pub primary: Hsla,
    /// Active primary background color.
    pub primary_active: Hsla,
    /// Primary text color.
    pub primary_foreground: Hsla,
    /// Hover primary background color.
    pub primary_hover: Hsla,
    /// Scrollbar background color.
    pub scrollbar: Hsla,
    /// Scrollbar thumb background color.
    pub scrollbar_thumb: Hsla,
    /// Scrollbar thumb hover background color.
    pub scrollbar_thumb_hover: Hsla,
    /// Secondary background color.
    pub secondary: Hsla,
    /// Active secondary background color.
    pub secondary_active: Hsla,
    /// Secondary text color, used for secondary Button text color or secondary text.
    pub secondary_foreground: Hsla,
    /// Hover secondary background color.
    pub secondary_hover: Hsla,
    /// Input selection background color.
    pub selection: Hsla,
    /// Skeleton background color.
    pub skeleton: Hsla,
    /// Success background color.
    pub success: Hsla,
    /// Success text color.
    pub success_foreground: Hsla,
    /// Success hover background color.
    pub success_hover: Hsla,
    /// Success active background color.
    pub success_active: Hsla,
    /// Transparent
    pub transparent: Hsla,
    /// Warning background color.
    pub warning: Hsla,
    /// Warning active background color.
    pub warning_active: Hsla,
    /// Warning hover background color.
    pub warning_hover: Hsla,
    /// Warning foreground color.
    pub warning_foreground: Hsla,
}

impl ThemeColor {
    pub fn light() -> Self {
        Self {
            accent: hsla(0., 0., 0.961, 1.),
            accent_foreground: hsla(0., 0., 0.09, 1.),
            background: hsla(0., 0., 1., 1.),
            border: hsla(0., 0., 0.898, 1.),
            caret: hsla(0., 0., 0.039, 1.),
            danger: Hsla::from(rgb(0xef4444)),        // Red 500
            danger_active: Hsla::from(rgb(0xdc2626)), // Red 600
            danger_hover: Hsla::from(rgb(0xef4444)).opacity(0.9), // Red 500
            danger_foreground: Hsla::from(rgb(0xfef2f2)), // Red 50
            foreground: hsla(0., 0., 0.039, 1.),
            info: Hsla::from(rgb(0x0ea5e9)),        // Sky 500
            info_active: Hsla::from(rgb(0x0284c7)), // Sky 600
            info_hover: Hsla::from(rgb(0x0ea5e9)).opacity(0.9), // Sky 500
            info_foreground: Hsla::from(rgb(0xf0f9ff)), // Sky 50
            input: hsla(0., 0., 0.898, 1.),         // same as border
            link: hsl(221.0, 83.0, 53.0),
            link_active: hsl(221.0, 83.0, 53.0).darken(0.2),
            link_hover: hsl(221.0, 83.0, 53.0).lighten(0.2),
            muted: hsla(0., 0., 0.961, 1.),
            muted_foreground: hsla(0., 0., 0.451, 1.),
            primary: hsla(0., 0., 0.09, 1.),
            primary_active: hsl(223.0, 1.9, 25.0),
            primary_foreground: hsl(223.0, 0.0, 98.0),
            primary_hover: hsl(223.0, 5.9, 15.0),
            scrollbar: hsl(0., 0., 92.).opacity(0.75),
            scrollbar_thumb: hsl(0., 0., 69.).opacity(0.9),
            scrollbar_thumb_hover: hsl(0., 0., 59.),
            secondary: hsla(0., 0., 0.961, 1.),
            secondary_active: hsl(240.0, 5.9, 93.),
            secondary_foreground: hsla(0., 0., 0.09, 1.),
            secondary_hover: hsl(240.0, 5.9, 98.),
            selection: hsl(211.0, 97.0, 85.0),
            skeleton: hsla(0., 0., 0.09, 1.).opacity(0.1),
            success: Hsla::from(rgb(0x22c55e)),        // Green 500
            success_active: Hsla::from(rgb(0x16a34a)), // Green 600
            success_hover: Hsla::from(rgb(0x22c55e)).opacity(0.9), // Green 500
            success_foreground: Hsla::from(rgb(0xf0fdf4)), // Green 50
            transparent: Hsla::transparent_black(),
            warning: Hsla::from(rgb(0xf59e0b)),        // Amber 500
            warning_active: Hsla::from(rgb(0xd97706)), // Amber 600
            warning_hover: Hsla::from(rgb(0xf59e0b)).opacity(0.9), // Amber 500
            warning_foreground: Hsla::from(rgb(0xfffbeb)), // Amber 50
        }
    }

    pub fn dark() -> Self {
        Self {
            accent: hsl(240.0, 3.7, 15.9),
            accent_foreground: hsl(0.0, 0.0, 78.0),
            background: hsl(0.0, 0.0, 8.0),
            border: hsl(240.0, 3.7, 16.9),
            caret: hsl(0., 0., 78.),
            danger: Hsla::from(rgb(0x991b1b)), // Red 800
            danger_active: Hsla::from(rgb(0x991b1b)).darken(0.2), // Red 800
            danger_foreground: Hsla::from(rgb(0xfef2f2)), // Red 50
            danger_hover: Hsla::from(rgb(0x991b1b)).opacity(0.9), // Red 800
            foreground: hsl(0., 0., 78.),
            info: Hsla::from(rgb(0x0c4a6e)), // Sky 900
            info_active: Hsla::from(rgb(0x0c4a6e)).darken(0.2), // Sky 900
            info_foreground: Hsla::from(rgb(0xf0f9ff)), // Sky 50
            info_hover: Hsla::from(rgb(0x0c4a6e)).opacity(0.8), // Sky 900
            input: hsl(240.0, 3.7, 15.9),
            link: hsl(221.0, 83.0, 53.0),
            link_active: hsl(221.0, 83.0, 53.0).darken(0.2),
            link_hover: hsl(221.0, 83.0, 53.0).lighten(0.2),
            muted: hsl(240.0, 3.7, 15.9),
            muted_foreground: hsl(240.0, 5.0, 34.),
            primary: hsl(223.0, 0.0, 98.0),
            primary_active: hsl(223.0, 0.0, 80.0),
            primary_foreground: hsl(223.0, 5.9, 10.0),
            primary_hover: hsl(223.0, 0.0, 90.0),
            scrollbar: hsl(240., 1., 15.).opacity(0.75),
            scrollbar_thumb: hsl(0., 0., 48.).opacity(0.9),
            scrollbar_thumb_hover: hsl(0., 0., 68.),
            secondary: hsl(240.0, 0., 13.0),
            secondary_active: hsl(240.0, 0., 13.),
            secondary_foreground: hsl(0.0, 0.0, 78.0),
            secondary_hover: hsl(240.0, 0., 15.),
            selection: hsl(211.0, 97.0, 22.0),
            skeleton: hsl(223.0, 0.0, 98.0).opacity(0.1),
            success: Hsla::from(rgb(0x166534)), // Green 800
            success_active: Hsla::from(rgb(0x166534)).darken(0.2), // Green 800
            success_foreground: Hsla::from(rgb(0xf0fdf4)), // Green 50
            success_hover: Hsla::from(rgb(0x166534)).opacity(0.8), // Green 800
            transparent: Hsla::transparent_black(),
            warning: Hsla::from(rgb(0x854d0e)), // Amber 800
            warning_active: Hsla::from(rgb(0x854d0e)).darken(0.2), // Amber 800
            warning_foreground: Hsla::from(rgb(0xfffbeb)), // Amber 50
            warning_hover: Hsla::from(rgb(0x854d0e)).opacity(0.9), // Amber 800
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    colors: ThemeColor,

    pub mode: ThemeMode,

    // Typography
    pub font_family: SharedString,
    pub heading_size: Pixels,
    pub text_size: Pixels,
    pub subtext_size: Pixels,
    pub line_height: Rems,
}

impl Deref for Theme {
    type Target = ThemeColor;

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.colors
    }
}

impl Global for Theme {}

impl Theme {
    pub fn init(cx: &mut App) {
        let _ = load_fonts(cx);
        let saved_theme = cx.global::<Storage>().get(StorageKey::SettingsTheme);

        let mode = match saved_theme {
            Some(theme) => match theme.as_str() {
                "light" => ThemeMode::Light,
                "dark" => ThemeMode::Dark,
                _ => ThemeMode::Light,
            },
            None => match cx.window_appearance() {
                WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeMode::Dark,
                WindowAppearance::Light | WindowAppearance::VibrantLight => ThemeMode::Light,
            },
        };

        // Sets theme to global if it's not already set
        Self::update(mode, None, cx);
    }

    /// Returns the global theme reference
    #[inline(always)]
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// Returns the global theme mutable reference
    #[inline(always)]
    pub fn global_mut(cx: &mut App) -> &mut Theme {
        cx.global_mut::<Theme>()
    }

    /// Returns true if the theme is dark.
    #[inline(always)]
    pub fn is_dark(&self) -> bool {
        self.mode.is_dark()
    }

    pub fn update(mode: ThemeMode, window: Option<&mut Window>, cx: &mut App) {
        let colors = match mode {
            ThemeMode::Light => ThemeColor::light(),
            ThemeMode::Dark => ThemeColor::dark(),
        };

        if !cx.has_global::<Theme>() {
            let theme = Theme::from(colors);
            cx.set_global(theme);
        }

        let theme = cx.global_mut::<Theme>();

        theme.mode = mode;
        theme.colors = colors;

        if let Some(window) = window {
            window.refresh();
        }
    }
}

impl From<ThemeColor> for Theme {
    fn from(colors: ThemeColor) -> Self {
        let mode = ThemeMode::default();
        Theme {
            mode,
            colors,

            font_family: "Inter".into(),
            heading_size: px(18.),
            text_size: px(14.),
            subtext_size: px(12.),
            line_height: rems(1.25),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq)]
pub enum ThemeMode {
    Light,
    #[default]
    Dark,
}

impl ThemeMode {
    #[inline(always)]
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}
