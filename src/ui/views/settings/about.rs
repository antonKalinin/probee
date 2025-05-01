use cargo_packager_updater::{semver::Version, url::Url, Update};
use gpui::*;

use crate::state::settings::*;
use crate::ui::{Button, ButtonVariants as _, Sizable as _, StyledExt, Theme};

pub struct AboutView {
    visible: bool,
    update: Option<Update>,
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
        }
    }

    fn check_updates(_event: &ClickEvent, _window: &mut Window, cx: &mut App) {
        cx.spawn(async move |_cx| {
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
                Ok(update) => {
                    if let Some(update) = update {
                        let on_chunk = |chunk_size, _chunk| {
                            println!("Downloaded {} bytes", chunk_size);
                        };

                        let on_download_finished = || println!("Download finished");

                        let update_result =
                            update.download_and_install_extended(on_chunk, on_download_finished);

                        match update_result {
                            Ok(_) => println!("Update installed successfully"),
                            Err(err) => println!("Failed to install update: {}", err),
                        }
                    } else {
                        println!("No update available")
                    }
                }
                Err(err) => {
                    println!("Failed to check for update: {}", err);
                }
            }
        })
        .detach();
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
            .gap_6()
            .flex()
            .flex_row()
            .items_start()
            .justify_center();

        let version_update = div().child(
            Button::new("check-updates-button")
                .label("Check for Updates")
                .small()
                .ghost()
                .on_click(AboutView::check_updates),
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
            .py_8()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(content_row.children(vec![logo, details]))
            // .child(section().child(row().children(vec![
            //     label("Version"),
            //     gapped().child(current_version).child(version_update),
            // ])))
            .into_any_element()
    }
}
