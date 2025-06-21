use gpui::{
    div, AnyView, App, Context, Entity, FocusHandle, InteractiveElement, IntoElement,
    ParentElement as _, Render, Styled, Window,
};

use crate::ui::{input::InputState, ActiveTheme};

/// Extension trait for [`WindowContext`] and [`ViewContext`] to add drawer functionality.
pub trait ContextModal: Sized {
    /// Return current focused Input entity.
    fn focused_input(&mut self, cx: &mut App) -> Option<Entity<InputState>>;
    /// Returns true if there is a focused Input entity.
    fn has_focused_input(&mut self, cx: &mut App) -> bool;
}

impl ContextModal for Window {
    fn has_focused_input(&mut self, cx: &mut App) -> bool {
        Root::read(self, cx).focused_input.is_some()
    }

    fn focused_input(&mut self, cx: &mut App) -> Option<Entity<InputState>> {
        Root::read(self, cx).focused_input.clone()
    }
}

/// Root is a view for the App window for as the top level view (Must be the first view in the window).
pub struct Root {
    /// Used to store the focus handle of the previous view.
    previous_focus_handle: Option<FocusHandle>,
    pub(super) focused_input: Option<Entity<InputState>>,
    view: AnyView,
}

impl Root {
    pub fn new(view: AnyView, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            previous_focus_handle: None,
            focused_input: None,
            view,
        }
    }

    pub fn update<F>(window: &mut Window, cx: &mut App, f: F)
    where
        F: FnOnce(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    {
        if let Some(Some(root)) = window.root::<Root>() {
            root.update(cx, |root, cx| f(root, window, cx));
        }
    }

    pub fn read<'a>(window: &'a Window, cx: &'a App) -> &'a Self {
        &window
            .root::<Root>()
            .expect("The window root view should be of type `ui::Root`.")
            .unwrap()
            .read(cx)
    }

    fn focus_back(&mut self, window: &mut Window, _: &mut App) {
        if let Some(handle) = self.previous_focus_handle.clone() {
            window.focus(&handle);
        }
    }

    /// Return the root view of the Root.
    pub fn view(&self) -> &AnyView {
        &self.view
    }
}

impl Render for Root {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("root")
            .relative()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.view.clone())
    }
}
