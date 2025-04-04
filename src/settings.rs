use gpui::{div, prelude::*, App, AppContext, Entity, Window};
use tab::*;

use crate::events::*;
use crate::state::settings::*;
// use crate::services::{Api, Auth, Storage};
// use crate::services::{AssistantConfig, User};
use crate::ui::*;

pub struct SettingsRoot {
    active_tab: SettingsTabType,

    login_view: Entity<LoginView>,
    profile_view: Entity<ProfileView>,
    error_view: Entity<ErrorView>,

    general_tab: Entity<SettingsTab>,
    profile_tab: Entity<SettingsTab>,
}

impl SettingsRoot {
    pub fn build(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        let state_controller = cx.global::<SettingsStateController>().clone();

        let view = cx.new(move |cx| {
            let state = state_controller.state.clone();

            let error_view = cx.new(|cx| ErrorView::new(cx, &state));
            let login_view = cx.new(|cx| LoginView::new(cx, &state));
            let profile_view = cx.new(|cx| ProfileView::new(cx, &state));

            let general_tab = cx.new(|_cx| SettingsTab::new(SettingsTabType::General, true));
            let profile_tab = cx.new(|_cx| SettingsTab::new(SettingsTabType::Profile, false));

            cx.subscribe(&general_tab, |_root, _this, event, cx| match event {
                SettingsEvent::SettingsTabSelected => {
                    set_active_tab(cx, SettingsTabType::General);
                }
                _ => {}
            });

            cx.subscribe(&profile_tab, |_root, _this, event, cx| match event {
                SettingsEvent::SettingsTabSelected => {
                    set_active_tab(cx, SettingsTabType::Profile);
                }
                _ => {}
            });

            SettingsRoot {
                active_tab: SettingsTabType::General,

                error_view,
                login_view,
                profile_view,

                general_tab,
                profile_tab,
            }
        });

        view
    }
}

impl Render for SettingsRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let title = div()
            .flex()
            .flex_row()
            .w_full()
            .h_8()
            .items_center()
            .justify_center()
            .text_sm()
            .text_color(theme.muted_foreground)
            .child("Settings");

        let tabs = div()
            .flex()
            .flex_row()
            .gap_1()
            .w_full()
            .h_16()
            .items_center()
            .justify_center()
            .border_b_1()
            .border_color(theme.border)
            .children([self.general_tab.clone(), self.profile_tab.clone()]);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .child(title)
            .child(tabs)
    }
}
