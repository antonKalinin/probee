use std::env;
use std::process::{exit, Command};
use std::time::{Duration, Instant};

use cargo_packager_updater::{semver::Version, url::Url, Update};
use gpui::{
    div, img, prelude::*, px, ClickEvent, Entity, ImageSource, SharedString, WeakEntity, Window,
};

use crate::state::settings::*;
use crate::ui::{Button, Sizable as _, StyledExt, Theme};

pub struct AboutView {
    visible: bool,
    update: Option<Update>,
    update_checked_at: Option<Instant>,
}

impl AboutView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<SettingsState>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::About;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::About;
            cx.notify();
        })
        .detach();

        AboutView {
            visible,
            update: None,
            update_checked_at: None,
        }
    }

    pub fn can_check_for_update(&mut self) -> bool {
        let now = Instant::now();
        let hour = Duration::from_secs(3600);

        match self.update_checked_at {
            Some(last_checked) => now.duration_since(last_checked) >= hour,
            None => true,
        }
    }

    fn check_updates(&self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn(async move |weak_view: WeakEntity<Self>, cx| {
            let pubkey = env!("CARGO_PACKAGER_SIGN_PUBLIC_KEY");
            let updates_url = format!(
                "{}/functions/v1/updates/{}",
                env!("SUPABASE_PUBLIC_URL"),
                "{{target}}/{{arch}}/{{current_version}}"
            );

            let config = cargo_packager_updater::Config {
                endpoints: vec![Url::parse(&updates_url).expect("Failed to parse updates URL")],
                pubkey: String::from(pubkey),
                ..Default::default()
            };

            let current_version =
                Version::parse(env!("CARGO_PKG_VERSION")).expect("Failed to parse version");

            let updater_builder =
                cargo_packager_updater::UpdaterBuilder::new(current_version.clone(), config)
                    .header(
                        "Authorization",
                        format!("Bearer {}", env!("SUPABASE_PUBLIC_ANON_KEY")),
                    )
                    .unwrap();

            let updater = updater_builder.build().unwrap();

            match updater.check() {
                Ok(app_update) => {
                    let _ = weak_view.update(cx, |view, cx| {
                        view.update = app_update;
                        view.update_checked_at = Some(Instant::now());
                        cx.notify();
                    });
                }
                Err(err) => {
                    // TODO: Handle error
                    println!("Failed to check for update: {}", err);
                }
            }
        })
        .detach();
    }

    fn install_update(&self, _event: &ClickEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        let update = self.update.clone();

        if update.is_none() {
            return;
        }

        let update_result = update.unwrap().download_and_install();

        match update_result {
            Ok(_) => self.restart_app(),
            // TODO: Handle error
            Err(err) => println!("Failed to install update: {}", err),
        }
    }

    fn restart_app(&self) {
        // Get path to the current executable
        let current_exe = match env::current_exe() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to get current executable path: {}", e);
                return;
            }
        };

        println!("Current executable path: {:?}", current_exe);

        let app_path = current_exe
            .ancestors()
            .find(|p| p.extension().map_or(false, |ext| ext == "app"));

        if app_path.is_none() {
            // restart app manually
            eprintln!("Failed to find app path");
            return;
        }

        // macos only
        let result = Command::new("open").arg(app_path.unwrap()).spawn();

        match result {
            Ok(_) => {
                exit(0);
            }
            Err(e) => {
                eprintln!("Failed to relaunch app: {}", e);
            }
        }
    }
}

impl Render for AboutView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let content_row = div()
            .w_full()
            .py_8()
            .gap_6()
            .flex()
            .flex_row()
            .items_start()
            .justify_center();

        let footer_row = div()
            .w_full()
            .gap_4()
            .pb_6()
            .flex()
            .flex_row()
            .items_center()
            .justify_center();

        let check_update_button = div().child(
            Button::new("check-updates-button")
                .label("Check for Updates")
                .small()
                .on_click(cx.listener({
                    |this, event, window, cx: &mut Context<Self>| {
                        this.check_updates(event, window, cx);
                    }
                })),
        );

        let install_update_button = div().child(
            Button::new("install-update-button")
                .label("Install Update & Restart")
                .small()
                .on_click(cx.listener({
                    |this, event, window, cx: &mut Context<Self>| {
                        this.install_update(event, window, cx);
                    }
                })),
        );

        let image_source: ImageSource = SharedString::new("images/icon_black_512.png").into();
        let logo = div().child(img(image_source).size_20());

        let details = div()
            .flex()
            .flex_col()
            .child(
                div()
                    .mb_2()
                    .text_size(theme.text_size)
                    .font_bold()
                    .child("Probee"),
            )
            .child(
                div()
                    .mb_1()
                    .text_size(theme.subtext_size)
                    .child(format!("Version {}", env!("CARGO_PKG_VERSION"))),
            )
            .child(
                div()
                    .flex_col()
                    .line_height(px(16.))
                    .text_color(theme.muted_foreground)
                    .text_size(theme.subtext_size)
                    .child(div().child("© Anton Kalinin"))
                    .child(div().child("2024-2025. All rights reserved.")), // TODO: Dynamic year
            );

        div()
            .w_full()
            .h_full()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(content_row.children(vec![logo, details]))
            .child(
                footer_row
                    .when(self.can_check_for_update(), |this| {
                        this.child(check_update_button)
                    })
                    .when(!self.can_check_for_update(), |this| {
                        this.text_size(theme.subtext_size)
                            .text_color(theme.muted_foreground)
                            .child("You have the latest version")
                    })
                    .when(self.update.is_some(), |this| {
                        this.child(install_update_button)
                    }),
            )
            .into_any_element()
    }
}
