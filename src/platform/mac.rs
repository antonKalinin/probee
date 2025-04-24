use std::path::PathBuf;

use anyhow::Result;
use cocoa::appkit::{
    NSImage, NSMenu, NSMenuItem, NSSquareStatusItemLength, NSStatusBar, NSStatusItem,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::runtime::Object;
use objc_id::Id;

static mut STATUS_ITEM: Option<Id<Object>> = None;
static mut STATUS_MENU: Option<Id<Object>> = None;

unsafe fn _ns_string(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string).autorelease() }
}

#[cfg(target_os = "macos")]
pub fn create_status_bar_item() -> Result<()> {
    use cocoa::appkit::NSImageView;

    unsafe {
        let icon_path = std::fs::canonicalize(PathBuf::from("./icons/icon_status_white@2x.png"))?;
        let icon_path_str = icon_path.to_str().unwrap();
        let ns_icon_path = NSString::alloc(nil).init_str(icon_path_str);
        let image: id = NSImage::alloc(nil).initByReferencingFile_(ns_icon_path);

        let status_item =
            NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSSquareStatusItemLength);

        // status_item.button().setTitle_(ns_string("🐝"));
        status_item.button().setImage_(image);

        let menu = NSMenu::new(nil).autorelease();

        let quit_title = NSString::alloc(nil).init_str("Quit");
        let quit_action = selector("terminate:");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                quit_title,
                quit_action,
                NSString::alloc(nil).init_str("q"),
            )
            .autorelease();
        menu.addItem_(quit_item);

        status_item.setMenu_(menu);

        STATUS_ITEM = Some(Id::from_ptr(status_item));
        STATUS_MENU = Some(Id::from_ptr(menu));

        Ok(())
    }
}
