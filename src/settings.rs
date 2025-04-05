use gpui::{div, prelude::*, App, AppContext, Entity, Window};
use settings::settings::SettingsView;
use tab::*;

use crate::state::settings::*;
// use crate::services::{Api, Auth, Storage};
// use crate::services::{AssistantConfig, User};
use crate::ui::*;

pub struct SettingsRoot {
    error_view: Entity<ErrorView>,
    general_view: Entity<SettingsView>,
    login_view: Entity<LoginView>,
    profile_view: Entity<ProfileView>,

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
            let general_view = cx.new(|cx| SettingsView::new(cx, &state));

            let general_tab = cx.new(|cx| SettingsTab::new(SettingsTabType::General, &state, cx));
            let profile_tab = cx.new(|cx| SettingsTab::new(SettingsTabType::Profile, &state, cx));

            SettingsRoot {
                error_view,
                general_view,
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

        // only one of the children should be visible per time
        let content = div().flex().w_full().p_2().children([
            div().child(self.general_view.clone()),
            div().child(self.login_view.clone()),
            div().child(self.profile_view.clone()),
        ]);

        let error = div().child(self.error_view.clone());

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .child(title)
            .child(tabs)
            .child(content)
            .child(error)
    }
}
