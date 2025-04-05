use gpui::*;

use crate::state::settings::*;
use crate::ui::Theme;

pub struct SettingsView {
    visible: bool,
}

impl SettingsView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<SettingsState>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::General;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::General;
            cx.notify();
        })
        .detach();

        SettingsView { visible }
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let _row = || div().w_full().flex_row();
        let _space = || div().flex().flex_grow().flex_shrink_0();
        let section = || div().flex_col().mb_2();
        let setting_title = || {
            div()
                .mb_4()
                .text_size(theme.text_size)
                .text_color(theme.primary)
                .font_weight(FontWeight::NORMAL)
        };

        div()
            .line_height(theme.line_height)
            .w_full()
            .p_1()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_sans.clone())
            .child(section().child(setting_title().child("Theme")))
            .child(section().child(setting_title().child("Position")))
            .into_any_element()
    }
}
