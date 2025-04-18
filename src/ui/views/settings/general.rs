use cargo_packager_updater::{semver::Version, url::Url, Update};
use gpui::*;

use crate::state::settings::*;
use crate::ui::{Button, ButtonVariants as _, Sizable as _, Theme};

pub struct GeneralSettingsView {
    visible: bool,
    update: Option<Update>,
}

impl GeneralSettingsView {
    pub fn new(cx: &mut Context<Self>, state: &Entity<SettingsState>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::General;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::General;
            cx.notify();
        })
        .detach();

        GeneralSettingsView {
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
                        println!("Update available")
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

impl Render for GeneralSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let _row = || div().w_full().flex_row();
        let _space = || div().flex().flex_grow().flex_shrink_0();
        let section = || div().flex_col().mb_2();

        let version_update = div().child(
            Button::new("check-updates-button")
                .label("Check for Updates")
                .small()
                .ghost()
                .on_click(GeneralSettingsView::check_updates),
        );

        div()
            .line_height(theme.line_height)
            .w_full()
            .p_1()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(section().child(version_update))
            .into_any_element()
    }
}
