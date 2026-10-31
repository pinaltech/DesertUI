//! AppKit places its titlebar above a full-size content view. A transparent,
//! right-aligned accessory forwards pointer input into GPUI, allowing Picacho's
//! exact 38px toolbar geometry without clicks being consumed by the native frame.
#[cfg(target_os = "macos")]
mod mac {
    #![allow(deprecated, unexpected_cfgs)]
    use cocoa::{
        base::{BOOL, NO, YES, id, nil},
        foundation::{NSPoint, NSRect, NSSize},
    };
    use objc::{
        class,
        declare::ClassDecl,
        msg_send,
        runtime::{Class, Object, Sel},
        sel, sel_impl,
    };
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use std::sync::Once;
    static REGISTER: Once = Once::new();
    unsafe fn target(this: &Object) -> id {
        unsafe { *this.get_ivar::<id>("desertUITarget") }
    }
    extern "C" fn mouse_down(this: &Object, _: Sel, event: id) {
        unsafe {
            let _: () = msg_send![target(this), mouseDown:event];
        }
    }
    extern "C" fn mouse_up(this: &Object, _: Sel, event: id) {
        unsafe {
            let _: () = msg_send![target(this), mouseUp:event];
        }
    }
    extern "C" fn mouse_moved(this: &Object, _: Sel, event: id) {
        unsafe {
            let _: () = msg_send![target(this), mouseMoved:event];
        }
    }
    extern "C" fn mouse_dragged(this: &Object, _: Sel, event: id) {
        unsafe {
            let _: () = msg_send![target(this), mouseDragged:event];
        }
    }
    extern "C" fn right_down(this: &Object, _: Sel, event: id) {
        unsafe {
            let _: () = msg_send![target(this), rightMouseDown:event];
        }
    }
    extern "C" fn right_up(this: &Object, _: Sel, event: id) {
        unsafe {
            let _: () = msg_send![target(this), rightMouseUp:event];
        }
    }
    extern "C" fn first_mouse(_: &Object, _: Sel, _: id) -> BOOL {
        YES
    }
    extern "C" fn first_responder(_: &Object, _: Sel) -> BOOL {
        NO
    }
    fn window(w: &gpui::Window) -> Option<(id, id)> {
        let handle = HasWindowHandle::window_handle(w).ok()?;
        if let RawWindowHandle::AppKit(handle) = handle.as_raw() {
            let view = handle.ns_view.as_ptr() as id;
            unsafe {
                let window: id = msg_send![view, window];
                if window != nil {
                    return Some((window, view));
                }
            }
        }
        None
    }
    /// Install once on a native window; do not call on TestAppContext windows.
    pub fn install(w: &gpui::Window) {
        let Some((window, view)) = window(w) else {
            return;
        };
        REGISTER.call_once(|| unsafe {
            let mut decl = ClassDecl::new("DesertUITitlebarInputView", class!(NSView)).unwrap();
            decl.add_ivar::<id>("desertUITarget");
            decl.add_method(
                sel!(mouseDown:),
                mouse_down as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(sel!(mouseUp:), mouse_up as extern "C" fn(&Object, Sel, id));
            decl.add_method(
                sel!(mouseMoved:),
                mouse_moved as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseDragged:),
                mouse_dragged as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(rightMouseDown:),
                right_down as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(rightMouseUp:),
                right_up as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(acceptsFirstMouse:),
                first_mouse as extern "C" fn(&Object, Sel, id) -> BOOL,
            );
            decl.add_method(
                sel!(acceptsFirstResponder),
                first_responder as extern "C" fn(&Object, Sel) -> BOOL,
            );
            decl.register();
        });
        unsafe {
            let accessory: id = msg_send![Class::get("DesertUITitlebarInputView").unwrap(), alloc];
            let frame = NSRect::new(
                NSPoint::new(0., 0.),
                NSSize::new(
                    (f64::from(w.viewport_size().width) - 80.).max(0.),
                    f64::from(crate::metrics::TITLEBAR),
                ),
            );
            let accessory: id = msg_send![accessory, initWithFrame: frame];
            (*accessory).set_ivar("desertUITarget", view);
            let controller: id = msg_send![class!(NSTitlebarAccessoryViewController), new];
            let _: () = msg_send![controller, setView: accessory];
            let _: () = msg_send![controller, setLayoutAttribute: 2isize]; // NSLayoutAttributeRight
            // GPUI's subclass hides native tab accessories. Call NSWindow's
            // implementation for this deliberate input accessory instead.
            let _: () = msg_send![super(window, class!(NSWindow)), addTitlebarAccessoryViewController: controller];
            let _: () = msg_send![accessory, release];
            let _: () = msg_send![controller, release];
        }
    }
    /// Resize the native accessory. Hosts should skip this in headless UI tests.
    pub fn sync_layout(w: &gpui::Window) {
        let Some((window, _)) = window(w) else { return };
        // Call only with native windows: GPUI's TestWindow raw-handle API panics.
        // Accessories are owned by NSWindow, with no global raw pointers.
        unsafe {
            let controllers: id = msg_send![window, titlebarAccessoryViewControllers];
            let count: usize = msg_send![controllers, count];
            for index in 0..count {
                let controller: id = msg_send![controllers, objectAtIndex:index];
                let view: id = msg_send![controller, view];
                let class = Class::get("DesertUITitlebarInputView");
                if class.is_some_and(|class| {
                    let yes: BOOL = msg_send![view, isKindOfClass:class];
                    yes == YES
                }) {
                    let frame: NSRect = msg_send![view, frame];
                    let size = NSSize::new(
                        (f64::from(w.viewport_size().width) - 80.).max(0.),
                        f64::from(crate::metrics::TITLEBAR),
                    );
                    if frame.size.width != size.width || frame.size.height != size.height {
                        let _: () = msg_send![view, setFrameSize:size];
                    }
                }
            }
        }
    }
}
#[cfg(target_os = "macos")]
pub use mac::{install, sync_layout};
#[cfg(not(target_os = "macos"))]
pub fn install(_: &gpui::Window) {}
#[cfg(not(target_os = "macos"))]
pub fn sync_layout(_: &gpui::Window) {}
