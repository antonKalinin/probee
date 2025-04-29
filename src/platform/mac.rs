use std::path::PathBuf;

use anyhow::Result;
use cocoa::appkit::{
    NSImage, NSImageView, NSMenu, NSMenuItem, NSSquareStatusItemLength, NSStatusBar, NSStatusItem,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

static mut HANDLER: Option<id> = None;
static mut STATUS_ITEM: Option<Id<Object>> = None;
static mut STATUS_MENU: Option<Id<Object>> = None;

#[cfg(target_os = "macos")]
pub fn register_selector() -> *const Class {
    unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("MenuHandler", superclass).unwrap();

        decl.add_method(
            sel!(activate:),
            activate_application as extern "C" fn(&Object, Sel, id),
        );

        decl.add_method(
            sel!(openSettings:),
            open_settings as extern "C" fn(&Object, Sel, id),
        );

        decl.register()
    }
}

unsafe fn _ns_string(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string).autorelease() }
}

pub fn create_status_item(handler: id) -> Result<()> {
    unsafe {
        HANDLER = Some(handler); // Store handler reference

        let icon_path =
            std::fs::canonicalize(PathBuf::from("./assets/images/icon_white_32x32.png"))?;
        let icon_path_str = icon_path.to_str().unwrap();
        let ns_icon_path = NSString::alloc(nil).init_str(icon_path_str);
        let image: id = NSImage::alloc(nil).initByReferencingFile_(ns_icon_path);
        let _: () = msg_send![image, setTemplate: true];

        let status_item =
            NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSSquareStatusItemLength);

        status_item.button().setImage_(image);

        let menu = NSMenu::new(nil).autorelease();

        let activate_title = NSString::alloc(nil).init_str("Show Probee");
        let activate_action = selector("activate:");
        let activate_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                activate_title,
                activate_action,
                NSString::alloc(nil).init_str(""),
            )
            .autorelease();
        activate_item.setTarget_(handler);
        menu.addItem_(activate_item);

        let settings_title = NSString::alloc(nil).init_str("Settings...");
        let settings_action = selector("openSettings:");
        let settings_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                settings_title,
                settings_action,
                NSString::alloc(nil).init_str(""),
            )
            .autorelease();
        settings_item.setTarget_(handler);
        menu.addItem_(settings_item);

        let separator = NSMenuItem::separatorItem(nil);
        menu.addItem_(separator);

        let quit_title = NSString::alloc(nil).init_str("Quit");
        let quit_action = selector("terminate:");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                quit_title,
                quit_action,
                NSString::alloc(nil).init_str(""),
            )
            .autorelease();
        menu.addItem_(quit_item);

        status_item.setMenu_(menu);

        STATUS_ITEM = Some(Id::from_ptr(status_item));
        STATUS_MENU = Some(Id::from_ptr(menu));

        Ok(())
    }
}

#[cfg(target_os = "macos")]
pub fn init_status_menu() -> Result<()> {
    unsafe {
        let handler_class = register_selector();
        let handler: id = msg_send![handler_class, new];

        create_status_item(handler)
    }
}

extern "C" fn activate_application(_this: &Object, _cmd: Sel, _notification: id) {
    println!("Application activated");
}

extern "C" fn open_settings(_this: &Object, _cmd: Sel, _notification: id) {
    println!("Open settings");
}
