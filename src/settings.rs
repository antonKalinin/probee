use gpui::{div, prelude::*, App, AppContext, Entity, FocusHandle, Window};

use crate::state::settings_state::*;
use crate::ui::*;
use crate::utils;

pub struct SettingsRoot {
    active_tab: SettingsTabType,

    // tabs content
    about_view: Entity<AboutView>,
    assistant_view: Entity<AssistantSettingsView>,
    general_view: Entity<GeneralSettingsView>,
    hotkeys_view: Entity<HotkeysView>,

    error_view: Entity<ErrorView>,
    tabs: Vec<Entity<SettingsTab>>,

    focus_handle: FocusHandle,
}

impl SettingsRoot {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Root> {
        let state_controller = cx.global::<SettingsStateController>().clone();

        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let view = cx.new(|cx| {
            let state = state_controller.state.clone();

            let about_view = cx.new(|cx| AboutView::new(cx, &state));
            let general_view = cx.new(|cx| GeneralSettingsView::new(&state, cx));
            let assistant_view = cx.new(|cx| AssistantSettingsView::new(&state, window, cx));
            let hotkeys_view = cx.new(|cx| HotkeysView::new(&state, cx));
            let error_view = cx.new(|cx| ErrorView::new(cx, &state));

            let tabs = vec![
                // cx.new(|cx| SettingsTab::new(SettingsTabType::General, &state, cx)),
                cx.new(|cx| SettingsTab::new(SettingsTabType::Assistant, &state, cx)),
                cx.new(|cx| SettingsTab::new(SettingsTabType::Hotkeys, &state, cx)),
                cx.new(|cx| SettingsTab::new(SettingsTabType::About, &state, cx)),
            ];

            cx.observe(&state, |this: &mut SettingsRoot, state, cx| {
                let data = state.read(cx);

                this.active_tab = data.active_tab.clone();
                cx.notify();
            })
            .detach();

            cx.on_blur(&focus_handle, window, |_this, _window, _cx| {
                // window.remove_window();
            })
            .detach();

            SettingsRoot {
                active_tab: state.read(cx).active_tab.clone(),

                about_view,
                assistant_view,
                general_view,
                hotkeys_view,

                tabs,
                error_view,

                focus_handle,
            }
        });

        cx.new(|cx| Root::new(view.into(), window, cx))
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

                window.set_frame(
                    utils::settings_window_bounds(cx, origin, next_height),
                    false,
                );
            })
            .when(self.active_tab == SettingsTabType::General, |this| {
                this.child(self.general_view.clone())
            })
            .when(self.active_tab == SettingsTabType::Assistant, |this| {
                this.child(self.assistant_view.clone())
            })
            .when(self.active_tab == SettingsTabType::Hotkeys, |this| {
                this.child(self.hotkeys_view.clone())
            })
            .when(self.active_tab == SettingsTabType::About, |this| {
                this.child(self.about_view.clone())
            });

        let error = div().child(self.error_view.clone());

        div()
            .track_focus(&self.focus_handle)
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
