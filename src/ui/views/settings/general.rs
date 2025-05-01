use gpui::*;

use crate::state::settings::*;
use crate::ui::{Checkbox, Theme};

pub struct GeneralSettingsView {
    visible: bool,

    startup_on_login: bool,
}

impl GeneralSettingsView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<SettingsState>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::General;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::General;
            cx.notify();
        })
        .detach();

        GeneralSettingsView {
            visible,
            startup_on_login: true,
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
                .mr_6()
                .child(text.to_owned())
        };

        let startup_launch_checkbox = Checkbox::new("startup-lauch")
            .label("Start Probee at login")
            .checked(true)
            .on_click(|checked, _window, cx| {});

        div()
            .w_full()
            .h(px(320.))
            .py_8()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(row().children(vec![label("Startup"), div().child(startup_launch_checkbox)]))
            .child(row().children(vec![label("Position")]))
            .child(row().children(vec![label("Theme")]))
            .into_any_element()
    }
}
