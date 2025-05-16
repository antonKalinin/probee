use gpui::prelude::FluentBuilder as _;
use gpui::*;

use crate::state::settings_state::*;
use crate::storage::*;
use crate::ui::{Icon, IconName, Theme, ThemeMode};

pub struct ThemeSwitch {
    active_theme: ThemeMode,
}

impl ThemeSwitch {
    pub fn new(_state: &Entity<SettingsState>, cx: &mut Context<Self>) -> Self {
        let storage = cx.global_mut::<Storage>();
        let saved_theme = storage
            .get(StorageKey::SettingsTheme)
            .unwrap_or("light".to_owned());

        let active_theme = match saved_theme.as_str() {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::Light,
        };

        ThemeSwitch { active_theme }
    }
}

impl Render for ThemeSwitch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let theme_button = |theme_mode: ThemeMode| {
            let active = theme_mode == self.active_theme;
            let on_click = cx.listener({
                move |this, _event, window, cx: &mut Context<Self>| {
                    this.active_theme = theme_mode;

                    let storage = cx.global_mut::<Storage>();

                    let value = match theme_mode {
                        ThemeMode::Light => "light",
                        ThemeMode::Dark => "dark",
                    };

                    let saved = storage.set(StorageKey::SettingsTheme, value.to_string());

                    if saved.is_ok() {
                        cx.notify();
                        Theme::update(theme_mode, Some(window), cx);
                    }
                }
            });

            let icon = (match theme_mode {
                ThemeMode::Light => Icon::new(IconName::Sun),
                ThemeMode::Dark => Icon::new(IconName::Moon),
            })
            .when(active, |this| this.text_color(theme.primary))
            .when(!active, |this| this.text_color(theme.muted_foreground));

            div()
                .w_16()
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .rounded_md()
                .cursor(CursorStyle::PointingHand)
                .on_mouse_down(MouseButton::Left, on_click)
                .when(active, |this| this.shadow_sm())
                .when(active, |this| this.bg(theme.background))
                .child(icon)
        };

        div()
            .w_auto()
            .h_9()
            .p_1()
            .gap_1()
            .flex()
            .flex_row()
            .items_center()
            .bg(theme.muted)
            .rounded_lg()
            .children(vec![
                theme_button(ThemeMode::Light),
                theme_button(ThemeMode::Dark),
            ])
    }
}
