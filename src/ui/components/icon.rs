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
    Apple,
    ArrowBigUp,
    BookA,
    BookMarked,
    BookType,
    BotMessageSquare,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronUp,
    CircleUserRound,
    CircleX,
    Command,
    Copy,
    Eraser,
    Eye,
    FilePlus,
    Globe,
    GraduationCap,
    HeartCrack,
    Inbox,
    Languages,
    Loader,
    LoaderCircle,
    MessageCircleX,
    MessageSquareX,
    Milk,
    Moon,
    Plus,
    Scan,
    Search,
    Send,
    Settings,
    Sigma,
    Signature,
    SpellCheck,
    Sun,
    Table,
    TableProperties,
    TextCursor,
    TextCursorInput,
    Trash,
    Trash2,
    TriangleAlert,
    WholeWord,
    X,

    // Custom
    Anthropic,
    OpenAi,
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_base_size = self.style.size.width.is_some() || self.style.size.height.is_some();

        let mut base = self.base;
        *base.style() = self.style;

        base.path(self.path)
            .flex_shrink_0()
            .when(!has_base_size, |this| this.size_4())
    }
}

impl From<Icon> for AnyElement {
    fn from(val: Icon) -> Self {
        val.into_any_element()
    }
}

impl Render for Icon {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let has_base_size = self.style.size.width.is_some() || self.style.size.height.is_some();

        let mut base = svg().flex_none();
        *base.style() = self.style.clone();

        base.path(self.path.clone())
            .flex_shrink_0()
            .when(!has_base_size, |this| this.size_4())
            .when_some(self.rotation, |this, rotation| {
                this.with_transformation(Transformation::rotate(rotation))
            })
    }
}
