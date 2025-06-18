use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::OnceLock;

use anyhow::Result;
use cocoa::appkit::{
    NSEventModifierFlags, NSImage, NSImageView, NSMenu, NSMenuItem, NSSquareStatusItemLength,
    NSStatusBar, NSStatusItem,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use gpui::App;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

static mut HANDLER: Option<id> = None;
static mut STATUS_ITEM: Option<Id<Object>> = None;
static mut STATUS_MENU: Option<Id<Object>> = None;
static MENU_SENDER: OnceLock<Sender<MenuAction>> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum MenuAction {
    OpenApp,
    OpenSettings,
}

#[cfg(target_os = "macos")]
pub fn register_selector() -> *const Class {
    unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("MenuHandler", superclass).unwrap();

        decl.add_method(sel!(openApp:), open_app as extern "C" fn(&Object, Sel, id));

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

fn create_status_item(handler: id) -> Result<()> {
    unsafe {
        HANDLER = Some(handler); // Store handler reference

        // Use NSBundle to get path to resource in .app bundle
        let main_bundle: id = msg_send![class!(NSBundle), mainBundle];
        let resource_name = NSString::alloc(nil).init_str("icon_white_32");
        let resource_type = NSString::alloc(nil).init_str("png");

        let mut icon_path: id =
            msg_send![main_bundle, pathForResource:resource_name ofType:resource_type];

        if icon_path == nil {
            let icon_path_buf =
                std::fs::canonicalize(PathBuf::from("./assets/images/icon_white_32.png"))?;

            icon_path = NSString::alloc(nil).init_str(icon_path_buf.to_str().unwrap());
        }

        let image: id = NSImage::alloc(nil).initByReferencingFile_(icon_path);
        let _: () = msg_send![image, setTemplate: true];

        let status_item =
            NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSSquareStatusItemLength);

        status_item.button().setImage_(image);

        let menu = NSMenu::new(nil).autorelease();

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

        // let separator = NSMenuItem::separatorItem(nil);
        // menu.addItem_(separator);

        let activate_title = NSString::alloc(nil).init_str("Open Probee");
        let activate_action = selector("openApp:");
        let activate_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                activate_title,
                activate_action,
                NSString::alloc(nil).init_str(""),
            )
            .autorelease();
        activate_item.setTarget_(handler);
        activate_item.setKeyEquivalentModifierMask_(NSEventModifierFlags::NSAlternateKeyMask);
        menu.addItem_(activate_item);

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
pub fn init_status_menu(_cx: &mut App) -> Result<Receiver<MenuAction>> {
    let (tx, rx) = mpsc::channel();

    MENU_SENDER
        .set(tx)
        .map_err(|_| anyhow::anyhow!("Menu sender already initialized"))?;

    unsafe {
        let handler_class = register_selector();
        let handler: id = msg_send![handler_class, new];

        create_status_item(handler)?;
    }

    Ok(rx)
}

fn send_menu_action(action: MenuAction) {
    if let Some(sender) = MENU_SENDER.get() {
        if let Err(e) = sender.send(action) {
            eprintln!("Failed to send menu action: {}", e);
        }
    } else {
        eprintln!("Menu sender not initialized");
    }
}

extern "C" fn open_app(_this: &Object, _cmd: Sel, _notification: id) {
    send_menu_action(MenuAction::OpenApp);
}

extern "C" fn open_settings(_this: &Object, _cmd: Sel, _notification: id) {
    send_menu_action(MenuAction::OpenSettings);
}
