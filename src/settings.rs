use gpui::{div, prelude::*, App, AppContext, Entity, Window};

use crate::state::settings::*;
use crate::ui::*;
use crate::utils;

pub struct SettingsRoot {
    active_tab: SettingsTabType,

    // tabs content
    about_view: Entity<AboutView>,
    general_view: Entity<GeneralSettingsView>,
    shortcuts_view: Entity<ShortcutsView>,

    error_view: Entity<ErrorView>,
    tabs: Vec<Entity<SettingsTab>>,
}

impl SettingsRoot {
    pub fn build(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        let state_controller = cx.global::<SettingsStateController>().clone();

        let view = cx.new(move |cx| {
            let state = state_controller.state.clone();

            let about_view = cx.new(|cx| AboutView::new(cx, &state));
            let general_view = cx.new(|cx| GeneralSettingsView::new(cx, &state));
            let shortcuts_view = cx.new(|cx| ShortcutsView::new(cx, &state));
            let error_view = cx.new(|cx| ErrorView::new(cx, &state));

            let tabs = vec![
                cx.new(|cx| SettingsTab::new(SettingsTabType::General, &state, cx)),
                cx.new(|cx| SettingsTab::new(SettingsTabType::Assistant, &state, cx)),
                cx.new(|cx| SettingsTab::new(SettingsTabType::Shortcuts, &state, cx)),
                cx.new(|cx| SettingsTab::new(SettingsTabType::About, &state, cx)),
            ];

            cx.observe(&state, |this: &mut SettingsRoot, state, cx| {
                let data = state.read(cx);

                this.active_tab = data.active_tab.clone();
                cx.notify();
            })
            .detach();

            SettingsRoot {
                active_tab: state.read(cx).active_tab.clone(),

                about_view,
                general_view,
                shortcuts_view,

                tabs,
                error_view,
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
            .child("Probee");

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
            .children(self.tabs.iter().map(|tab| tab.clone()));

        let content = div()
            .w_full()
            .on_children_prepainted(move |bounds, window, cx| {
                let content_height: f32 = bounds.iter().map(|b| b.size.height.0).sum();
                let next_height = 32. + 64. + content_height; // title + tabs + content
                let origin = window.bounds().origin;

                window.set_frame(utils::settings_window_bounds(cx, origin, next_height));
            })
            .when(self.active_tab == SettingsTabType::General, |this| {
                this.child(self.general_view.clone())
            })
            .when(self.active_tab == SettingsTabType::Shortcuts, |this| {
                this.child(self.shortcuts_view.clone())
            })
            .when(self.active_tab == SettingsTabType::About, |this| {
                this.child(self.about_view.clone())
            });

        let error = div().child(self.error_view.clone());

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .child(title)
            .child(tabs)
            .child(content)
            .child(error) // Error should be within content
    }
}
