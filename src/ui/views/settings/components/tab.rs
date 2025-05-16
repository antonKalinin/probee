use gpui::*;

use crate::state::settings_state::*;
use crate::ui::{Icon, IconName, Theme};

pub struct SettingsTab {
    active: bool,
    tab_type: SettingsTabType,
}

impl SettingsTab {
    pub fn new(
        tab_type: SettingsTabType,
        state: &Entity<SettingsState>,
        cx: &mut Context<Self>,
    ) -> Self {
        let active = state.read(cx).active_tab == tab_type;

        cx.observe(state, |this, state, cx| {
            this.active = state.read(cx).active_tab == this.tab_type;
            cx.notify();
        })
        .detach();

        SettingsTab { active, tab_type }
    }

    fn render_icon(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let icon = match self.tab_type {
            SettingsTabType::General => Icon::new(IconName::Settings),
            SettingsTabType::Assistant => Icon::new(IconName::BotMessageSquare),
            SettingsTabType::Shortcuts => Icon::new(IconName::Command),
            SettingsTabType::About => Icon::new(IconName::Signature),
        };

        let text_color = match self.active {
            true => theme.secondary_foreground,
            false => theme.muted_foreground,
        };

        icon.group_hover("settings-tab", |style| {
            style.text_color(theme.secondary_foreground)
        })
        .text_color(text_color)
    }

    fn render_label(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let text = match self.tab_type {
            SettingsTabType::General => "General",
            SettingsTabType::Assistant => "Assistant",
            SettingsTabType::Shortcuts => "Shortcuts",
            SettingsTabType::About => "About",
        };

        let text_color = match self.active {
            true => theme.secondary_foreground,
            false => theme.muted_foreground,
        };

        let label = div()
            .group_hover("settings-tab", |style| {
                style.text_color(theme.secondary_foreground)
            })
            .flex()
            .pt_1()
            .text_xs()
            .text_color(text_color)
            .child(text);

        label.into_any_element()
    }
}

impl Render for SettingsTab {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let on_click = cx.listener({
            move |this, _event, _window, cx: &mut Context<Self>| {
                set_active_tab(cx, this.tab_type.clone());
            }
        });

        let bg_color = match self.active {
            true => theme.muted,
            false => theme.background,
        };

        let button = div()
            .group("settings-tab")
            .w_16()
            .px_2()
            .py_1()
            .flex()
            .flex_col()
            .items_center()
            .bg(bg_color)
            .rounded_md()
            .on_mouse_up(MouseButton::Left, on_click)
            .cursor(CursorStyle::PointingHand)
            .child(self.render_icon(cx))
            .child(self.render_label(cx));

        button
    }
}
