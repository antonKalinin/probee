use anyhow::Result;
use cocoa::appkit::{
    NSImage, NSMenu, NSMenuItem, NSSquareStatusItemLength, NSStatusBar, NSStatusItem, NSWindow,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::runtime::Object;
use objc_id::Id;

static mut STATUS_ITEM: Option<Id<Object>> = None;
static mut STATUS_MENU: Option<Id<Object>> = None;

unsafe fn ns_string(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string).autorelease() }
}

#[cfg(target_os = "macos")]
pub fn create_status_bar_item() -> Result<()> {
    unsafe {
        // let image: id = msg_send![class!(NSImage), alloc];
        // image.initWithContentsOfFile_(NSString::alloc(nil).init_str("test.jpeg"));

        let status_item =
            NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSSquareStatusItemLength);

        status_item.button().setTitle_(ns_string("🐝"));

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

#[cfg(not(target_os = "macos"))]
pub fn create_status_bar_item() -> Result<(), Box<dyn Error>> {
    Ok(())
}
