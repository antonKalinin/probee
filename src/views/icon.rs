use std::fmt;

use gpui::SharedString;

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

impl Icon {
    pub fn path(&self) -> SharedString {
        let name = to_kebap(self.to_string().as_str());
        SharedString::from(format!("icons/{}.svg", name))
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    BookA,
    BookMarked,
    BookType,
    CircleX,
    Command,
    Eraser,
    FilePlus,
    Globe,
    GraduationCap,
    Languages,
    Loader,
    MessageCircleX,
    MessageSquareX,
    Milk,
    Plus,
    Scan,
    Send,
    Sigma,
    SpellCheck,
    Table,
    TableProperties,
    TextCursor,
    TextCursorInput,
    WholeWord,
    X,
}
