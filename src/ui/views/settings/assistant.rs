use gpui::*;

use crate::state::settings::*;
use crate::ui::{Dropdown, SearchableVec, TextInput, Theme};

pub struct AssistantSettingsView {
    api_key_input: Entity<TextInput>,
    model_dropdown: Entity<Dropdown<SearchableVec<SharedString>>>,
    visible: bool,
}

impl AssistantSettingsView {
    pub fn new(state: &Entity<SettingsState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let data = state.read(cx);
        let visible = data.active_tab == SettingsTabType::Assistant;

        cx.observe(state, |this, state, cx| {
            let data = state.read(cx);
            this.visible = data.active_tab == SettingsTabType::Assistant;
            cx.notify();
        })
        .detach();

        let fruits = SearchableVec::new(vec![
            "Apple".into(),
            "Orange".into(),
            "Banana".into(),
            "Grape".into(),
            "Pineapple".into(),
            "Watermelon & This is a long long long long long long long long long title".into(),
            "Avocado".into(),
        ]);

        let api_key_input =
            cx.new(|cx| TextInput::new(window, cx).placeholder("Enter Anthropic API Key"));

        let model_dropdown = cx.new(|cx| {
            Dropdown::new("model-dropdown", fruits, None, window, cx).placeholder("Select Model")
        });

        AssistantSettingsView {
            api_key_input,
            model_dropdown,
            visible,
        }
    }
}

impl Render for AssistantSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        if !self.visible {
            return div().into_any_element();
        }

        let row = || div().w_full().flex().flex_row().mb_6().items_center();

        let label = |text: &str| {
            div()
                .w(px(168.))
                .text_align(TextAlign::Right)
                .text_size(theme.subtext_size)
                .text_color(theme.muted_foreground)
                .font_weight(FontWeight::MEDIUM)
                .mr_6()
                .child(text.to_owned())
        };

        let value = || div().w(px(280.));

        div()
            .w_full()
            .h(px(400.))
            .py_8()
            .text_color(theme.foreground)
            .text_size(theme.text_size)
            .line_height(theme.line_height)
            .font_family(theme.font_family.clone())
            .child(row().children(vec![
                label("Model"),
                value().child(self.model_dropdown.clone()),
            ]))
            .child(row().children(vec![
                label("API Key"),
                value().child(self.api_key_input.clone()),
            ]))
            .into_any_element()
    }
}
