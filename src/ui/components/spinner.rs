use std::time::Duration;

use crate::ui::{Icon, IconName};
use gpui::{
    ease_in_out, percentage, prelude::FluentBuilder as _, px, Animation, AnimationExt as _, App,
    Hsla, IntoElement, Pixels, RenderOnce, Styled as _, Transformation, Window,
};

#[derive(IntoElement)]
pub struct Spinner {
    icon: Icon,
    speed: Duration,
    size: Pixels,
    color: Option<Hsla>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            speed: Duration::from_secs_f64(0.8),
            icon: Icon::new(IconName::LoaderCircle),
            color: None,
            size: px(16.),
        }
    }

    #[allow(dead_code)]
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.icon
            .size(self.size)
            .when_some(self.color, |this, color| this.text_color(color))
            .with_animation(
                "circle",
                Animation::new(self.speed).repeat().with_easing(ease_in_out),
                |this, delta| this.with_transformation(Transformation::rotate(percentage(delta))),
            )
            .into_element()
    }
}
