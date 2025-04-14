use std::time::Duration;

use crate::ui::{Icon, IconName};
use gpui::{
    div, ease_in_out, percentage, prelude::FluentBuilder as _, Animation, AnimationExt as _, App,
    Hsla, IntoElement, ParentElement, RenderOnce, Styled as _, Transformation, Window,
};

#[derive(IntoElement)]
pub struct Spinner {
    icon: Icon,
    speed: Duration,
    color: Option<Hsla>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            speed: Duration::from_secs_f64(0.8),
            icon: Icon::new(IconName::LoaderCircle),
            color: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.icon
            .when_some(self.color, |this, color| this.text_color(color))
            .with_animation(
                "circle",
                Animation::new(self.speed).repeat().with_easing(ease_in_out),
                |this, delta| this.with_transformation(Transformation::rotate(percentage(delta))),
            )
            .into_element()
    }
}
