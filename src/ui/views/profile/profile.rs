use gpui::*;

use super::utils::*;
use crate::services::{Auth, User};
use crate::state::settings::*;
use crate::ui::Theme;

pub struct ProfileView {
    user: Option<User>,
    visible: bool,
}

pub fn get_greeting(name: Option<String>) -> String {
    let name = name.unwrap_or("Besty".into());
    let now = get_local_time_in_secs();

    // Convert to hour of day (0-23)
    let hour = ((now % 86400) / 3600) as u8;

    let greeting = match hour {
        5..=11 => "Good morning",
        12..=16 => "Good afternoon",
        17..=21 => "Good evening",
        _ => "Good night",
    };

    format!("{}, {}", greeting, name)
}

impl ProfileView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<SettingsState>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::Profile && data.authenticated;
        let user = state.read(cx).user.clone();

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::Profile && data.authenticated;
            this.user = data.user.clone();
            cx.notify();
        })
        .detach();

        ProfileView { visible, user }
    }
}

impl Render for ProfileView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let handle_logout = cx.listener(move |_this, _event, _window, cx: &mut Context<Self>| {
            let auth = cx.global::<Auth>().clone();

            cx.spawn(async move |_this, cx| {
                let logout_result = auth.logout(cx).await;

                match logout_result {
                    Ok(_) => {
                        set_user_async(cx, None);
                        set_authenticated_async(cx, false);
                    }
                    Err(err) => {
                        set_error_async(cx, Some(err));
                    }
                };
            })
            .detach();
        });

        let logout_button = div()
            .mt_4()
            .px_4()
            .py_2()
            .rounded_lg()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .bg(theme.background)
            .border_1()
            .border_color(theme.border)
            .text_color(theme.secondary_foreground)
            .hover(|style| style.bg(theme.secondary))
            .on_mouse_up(MouseButton::Left, handle_logout)
            .cursor_pointer()
            .child("Logout");

        let first_name = self
            .user
            .as_ref()
            .map(|user| user.full_name.clone())
            .map(|name| name.split(' ').next().unwrap().to_string());

        let greeting = div()
            .mb_4()
            .flex()
            .text_size(theme.heading_size)
            .text_color(theme.foreground)
            .font_weight(FontWeight::SEMIBOLD)
            .child(get_greeting(first_name));

        let row = || div().w_full().flex_row();
        let space = || div().flex().flex_grow().flex_shrink_0();
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
            .child(greeting)
            .child(section().child(setting_title().child("Theme")))
            .child(section().child(setting_title().child("Position")))
            .child(row().children([space(), logout_button]))
            .into_any_element()
    }
}
