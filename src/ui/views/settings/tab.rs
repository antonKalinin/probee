use gpui::*;

use crate::events::UiEvent;
use crate::ui::{Icon, Theme};

#[derive(Debug, Clone, PartialEq)]
pub enum TabType {
    General,
    Profile,
}

pub struct SettingsTab {
    active: bool,
    tab_type: TabType,
}

impl SettingsTab {
    pub fn new(tab_type: TabType, active: bool) -> Self {
        SettingsTab { active, tab_type }
    }

    fn render_icon(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let icon = match self.tab_type {
            TabType::General => Icon::Settings,
            TabType::Profile => Icon::CircleUserRound,
        };

        let text_color = match self.active {
            true => theme.secondary_foreground,
            false => theme.muted_foreground,
        };

        let svg = div().flex().child(
            svg()
                .group_hover("settings-tab", |style| {
                    style.text_color(theme.secondary_foreground)
                })
                .path(icon.path())
                .text_color(text_color)
                .size_4(),
        );

        svg.into_any_element()
    }

    fn render_label(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let text = match self.tab_type {
            TabType::General => "General",
            TabType::Profile => "Profile",
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
                // cx.emit(UiEvent::ChangeAssistant(assistant_id));
            }
        });

        let bg_color = match self.active {
            true => theme.muted,
            false => theme.background,
        };

        let button = div()
            .group("settings-tab")
            .w_auto()
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

impl EventEmitter<UiEvent> for SettingsTab {}
