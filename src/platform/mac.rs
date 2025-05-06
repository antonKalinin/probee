use std::path::PathBuf;

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

const MAC_PLATFORM_IVAR: &str = "platform";

#[cfg(target_os = "macos")]
pub fn register_selector() -> *const Class {
    unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("MenuHandler", superclass).unwrap();

        decl.add_method(sel!(openApp:), open_app as extern "C" fn(&Object, Sel, id));

        decl.add_method(
            sel!(checkUpdates:),
            check_updates as extern "C" fn(&Object, Sel, id),
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

        let separator = NSMenuItem::separatorItem(nil);
        menu.addItem_(separator);

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

        let check_updates_title = NSString::alloc(nil).init_str("Check for Updates");
        let check_updates_action = selector("checkUpdates:");
        let check_updates_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(
                check_updates_title,
                check_updates_action,
                NSString::alloc(nil).init_str(""),
            )
            .autorelease();
        check_updates_item.setTarget_(handler);
        menu.addItem_(check_updates_item);

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
pub fn init_status_menu(_cx: &mut App) -> Result<()> {
    unsafe {
        let handler_class = register_selector();
        let handler: id = msg_send![handler_class, new];

        create_status_item(handler)
    }
}

extern "C" fn open_app(_this: &Object, _cmd: Sel, _notification: id) {
    println!("Open app");
}

extern "C" fn check_updates(_this: &Object, _cmd: Sel, _notification: id) {
    println!("Check updates");
}

extern "C" fn open_settings(_this: &Object, _cmd: Sel, _notification: id) {
    println!("Open settings");
}

// unsafe fn get_mac_platform(object: &mut Object) -> &MacPlatform {
//     unsafe {
//         let platform_ptr: *mut c_void = *object.get_ivar(MAC_PLATFORM_IVAR);
//         assert!(!platform_ptr.is_null());
//         &*(platform_ptr as *const MacPlatform)
//     }
// }

// extern "C" fn handle_status_item(this: &mut Object, _cmd: Sel, item: id) {
//     unsafe {
//         let platform = get_mac_platform(this);
//         let mut lock = platform.0.lock();
//         if let Some(mut callback) = lock.menu_command.take() {
//             let tag: NSInteger = msg_send![item, tag];
//             let index = tag as usize;
//             if let Some(action) = lock.menu_actions.get(index) {
//                 let action = action.boxed_clone();
//                 drop(lock);
//                 callback(&*action);
//             }
//             platform.0.lock().menu_command.get_or_insert(callback);
//         }
//     }
// }
