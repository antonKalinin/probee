use anyhow::Error;
use gpui::{div, prelude::*, App, AppContext, AsyncApp, Entity, Global, Window};
use tab::*;

use crate::events::*;
// use crate::services::{Api, Auth, Storage};
// use crate::services::{AssistantConfig, User};
use crate::state::*;
use crate::ui::*;

#[derive(Debug)]
pub struct State {
    pub active_tab: TabType,
    pub error: Option<Error>,
    pub loading: bool,
}

#[derive(Clone)]
pub struct SettingsState {
    pub state: Entity<State>,
}

impl Global for SettingsState {}

impl SettingsState {
    pub fn init(cx: &mut App) {
        let state: Entity<State> = cx.new(|_cx| State {
            active_tab: TabType::General,
            error: None,
            loading: false,
        });

        let settings_state = SettingsState { state };

        cx.set_global(settings_state);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut App), cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        })
    }

    pub fn update_async(f: impl FnOnce(&mut Self, &mut App), cx: &mut AsyncApp) {
        let _ = cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        });
    }

    pub fn set_active_tab(&self, cx: &mut App, tab: TabType) {
        self.state.update(cx, |state, cx| {
            state.active_tab = tab;
            cx.notify();
        });
    }
}

pub struct SettingsRoot {
    active_tab: TabType,

    login_view: Entity<LoginView>,
    profile_view: Entity<ProfileView>,
    error_view: Entity<ErrorView>,

    general_tab: Entity<SettingsTab>,
    profile_tab: Entity<SettingsTab>,
}

impl SettingsRoot {
    pub fn build(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        let settings_state = cx.global::<GlobalState>().clone();

        let view = cx.new(move |cx| {
            let state = settings_state.state.clone();

            let error_view = cx.new(|cx| ErrorView::new(cx, &state));
            let login_view = cx.new(|cx| LoginView::new(cx, &state));
            let profile_view = cx.new(|cx| ProfileView::new(cx, &state));

            let general_tab = cx.new(|_cx| SettingsTab::new(TabType::General, true));
            let profile_tab = cx.new(|_cx| SettingsTab::new(TabType::Profile, false));

            SettingsRoot {
                active_tab: TabType::General,

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
