use gpui::{
    div, rems, AnyElement, App, AppContext, Context, ElementId, Entity, IntoElement, ParentElement,
    Render, RenderOnce, Styled, Window,
};

use crate::assistant::Prompt;
use crate::ui::{h_flex, Checkbox, Icon, IconName, List, ListDelegate, ListItem, Theme};

#[derive(IntoElement)]
pub struct PromptListItem {
    base: ListItem,
    ix: usize,
    prompt: Prompt,
    selected: bool,
}

impl RenderOnce for PromptListItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let propmt_enabled_checkbox = Checkbox::new(self.ix)
            .checked(true)
            .on_click(|checked, _window, _cx| {});

        let readonly_icon = Icon::new(IconName::PencilOff)
            .size_3()
            .text_color(theme.muted_foreground);

        self.base
            .px_3()
            .py_1()
            .overflow_x_hidden()
            .cursor_pointer()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .items_center()
                            .justify_start()
                            .gap_2()
                            .child(propmt_enabled_checkbox)
                            .child(self.prompt.name.clone()),
                    )
                    .child(readonly_icon),
            )
    }
}

impl PromptListItem {
    pub fn new(id: impl Into<ElementId>, prompt: Prompt, ix: usize, selected: bool) -> Self {
        PromptListItem {
            prompt,
            ix,
            base: ListItem::new(id),
            selected,
        }
    }
}

struct PromptListDelegate {
    prompts: Vec<Prompt>,
    on_select: Option<Box<dyn Fn(&Prompt)>>,
    selected_index: Option<usize>,
}

impl PromptListDelegate {
    fn selected_prompt(&self) -> Option<Prompt> {
        let Some(ix) = self.selected_index else {
            return None;
        };

        self.prompts.get(ix).cloned()
    }
}

impl ListDelegate for PromptListDelegate {
    type Item = PromptListItem;

    fn items_count(&self, _: &App) -> usize {
        self.prompts.len()
    }

    fn set_selected_index(
        &mut self,
        ix: Option<usize>,
        _: &mut Window,
        cx: &mut Context<List<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<List<Self>>) {
        println!("Index {} confirmed", self.selected_index.unwrap_or(0));
    }

    fn render_item(
        &self,
        ix: usize,
        _: &mut Window,
        _: &mut Context<List<Self>>,
    ) -> Option<Self::Item> {
        let selected = Some(ix) == self.selected_index;
        if let Some(prompt) = self.prompts.get(ix) {
            return Some(PromptListItem::new(ix, prompt.clone(), ix, selected));
        }

        None
    }

    fn render_initial(
        &self,
        _window: &mut Window,
        _cx: &mut Context<List<Self>>,
    ) -> Option<AnyElement> {
        Some(div().w_full().h_6().child("Loading...").into_any_element())
    }
}

pub struct PromptList {
    prompt_list: Entity<List<PromptListDelegate>>,
    on_select: Option<Box<dyn Fn(&Prompt)>>,
}

impl PromptList {
    pub fn new(prompts: Vec<Prompt>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = PromptListDelegate {
            prompts,
            selected_index: None,
            on_select: None,
        };

        let prompt_list = cx.new(|cx| {
            List::new(delegate, window, cx)
                .no_query()
                .selectable(true)
                .max_h(rems(11.5))
        });

        PromptList {
            prompt_list,
            on_select: None,
        }
    }
}

impl Render for PromptList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex_1()
            .w_full()
            .border_1()
            .h(rems(11.5))
            .border_color(theme.border)
            .rounded_lg()
            .child(self.prompt_list.clone())
    }
}
