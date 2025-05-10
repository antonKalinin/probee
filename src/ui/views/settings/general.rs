use gpui::*;

use crate::state::settings::*;
use crate::ui::{Checkbox, Theme};

use super::components::ThemeSwitch;

pub struct GeneralSettingsView {
    visible: bool,
    startup_on_login: bool,

    theme_switch: Entity<ThemeSwitch>,
}

impl GeneralSettingsView {
    pub fn new(state: &Entity<SettingsState>, cx: &mut Context<Self>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::General;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::General;
            cx.notify();
        })
        .detach();

        let theme_switch = cx.new(|cx| ThemeSwitch::new(&state, cx));

        GeneralSettingsView {
            visible,
            startup_on_login: true,
            theme_switch,
        }
    }
}

impl Render for GeneralSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let row = || div().w_full().flex().flex_row().mb_6().items_center();

        let label = |text: &str| {
            div()
                .w(px(232.))
                .text_align(TextAlign::Right)
                .text_size(theme.subtext_size)
                .text_color(theme.muted_foreground)
                .font_weight(FontWeight::MEDIUM)
                .mr_6()
                .child(text.to_owned())
        };

        let handle_startup = cx.listener(|this, value: &bool, _window, cx| {
            this.startup_on_login = value.clone();
            cx.notify();
        });

        let startup_launch_checkbox = Checkbox::new("startup-lauch")
            .label("Start Probee at login")
            .checked(self.startup_on_login)
            .on_click(handle_startup);

        div()
            .w_full()
            .h(px(320.))
            .py_8()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(row().children(vec![label("Startup"), div().child(startup_launch_checkbox)]))
            .child(row().children(vec![label("Theme"), div().child(self.theme_switch.clone())]))
            .into_any_element()
    }
}
