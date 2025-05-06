use gpui::App;

pub mod actions;
pub mod button;
pub mod checkbox;
pub mod dropdown;
pub mod error;
pub mod header;
pub mod icon;
pub mod input;
pub mod list;
pub mod scroll;
pub mod skeleton;
pub mod spinner;

pub use button::*;
pub use checkbox::*;
pub use dropdown::*;
pub use error::*;
pub use header::*;
pub use icon::*;
pub use input::*;
pub use list::*;
pub use scroll::*;
pub use skeleton::*;
pub use spinner::*;

pub struct Components;

impl Components {
    pub fn init(cx: &mut App) {
        input::init(cx);
        dropdown::init(cx);
    }
}
