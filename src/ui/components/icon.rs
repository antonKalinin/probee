use std::fmt;

use gpui::{
    prelude::FluentBuilder as _, svg, AnyElement, App, Context, InteractiveElement, IntoElement,
    Radians, Render, RenderOnce, SharedString, StyleRefinement, Styled, Svg, Transformation,
    Window,
};

fn to_kebap(s: &str) -> String {
    s.chars().fold(String::new(), |mut s, c| {
        if c.is_uppercase() || c.is_numeric() {
            if !s.is_empty() {
                s.push('-');
            }
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c);
        }
        s
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IconName {
    ArrowBigUp,
    BookA,
    BookMarked,
    BookType,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronUp,
    CircleUserRound,
    CircleX,
    Command,
    Copy,
    Eraser,
    FilePlus,
    Globe,
    GraduationCap,
    HeartCrack,
    Languages,
    Loader,
    LoaderCircle,
    MessageCircleX,
    MessageSquareX,
    Milk,
    Plus,
    Scan,
    Send,
    Settings,
    Sigma,
    SpellCheck,
    Table,
    TableProperties,
    TextCursor,
    TextCursorInput,
    Trash,
    Trash2,
    TriangleAlert,
    WholeWord,
    X,
}

impl IconName {
    pub fn path(self) -> SharedString {
        let name = to_kebap(self.to_string().as_str());
        SharedString::from(format!("icons/{}.svg", name))
    }
}

impl fmt::Display for IconName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<IconName> for Icon {
    fn from(value: IconName) -> Self {
        Icon::new(value)
    }
}

impl From<IconName> for AnyElement {
    fn from(value: IconName) -> Self {
        Icon::new(value).into_any_element()
    }
}

#[derive(IntoElement)]
pub struct Icon {
    base: Svg,
    path: SharedString,
    rotation: Option<Radians>,
    style: StyleRefinement,
}

impl Icon {
    pub fn new(icon: IconName) -> Self {
        Icon {
            base: svg().flex_none(),
            path: icon.path(),
            rotation: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn with_transformation(mut self, transformation: Transformation) -> Self {
        self.base = self.base.with_transformation(transformation);
        self
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for Icon {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Icon {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut base = self.base;
        *base.style() = self.style;

        base.flex_shrink_0().path(self.path).size_full()
    }
}

impl From<Icon> for AnyElement {
    fn from(val: Icon) -> Self {
        val.into_any_element()
    }
}

impl Render for Icon {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut base = svg().flex_none();
        *base.style() = self.style.clone();

        base.path(self.path.clone())
            .size_full()
            .when_some(self.rotation, |this, rotation| {
                this.with_transformation(Transformation::rotate(rotation))
            })
    }
}
