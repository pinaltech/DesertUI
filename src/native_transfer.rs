//! Platform file drags and trackpad magnification for GPUI library surfaces.
// Native implementation below; callers retain in-app drop semantics.
#[cfg(not(target_os = "macos"))]
pub fn start_file_drag(_: &gpui::Window, _: &[std::path::PathBuf]) -> bool {
    false
}
#[cfg(not(target_os = "macos"))]
pub fn install_magnify(_: &gpui::Window) {}
#[cfg(target_os = "macos")]
mod mac {
    #![allow(deprecated, unexpected_cfgs)]
    use cocoa::{
        base::{BOOL, YES, id, nil},
        foundation::{NSPoint, NSRect, NSSize, NSString},
    };
    use objc::{
        class,
        declare::ClassDecl,
        msg_send,
        runtime::{Class, Object, Protocol, Sel},
        sel, sel_impl,
    };
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use std::{cell::RefCell, collections::HashMap, path::PathBuf, sync::Once};
    thread_local! { static MAGNIFY: RefCell<HashMap<usize, super::MagnifyGesture>> = RefCell::default(); }
    fn view(w: &gpui::Window) -> Option<id> {
        if let RawWindowHandle::AppKit(h) = HasWindowHandle::window_handle(w).ok()?.as_raw() {
            Some(h.ns_view.as_ptr() as id)
        } else {
            None
        }
    }
    extern "C" fn magnify(this: &Object, _: Sel, event: id) {
        let delta: f64 = unsafe { msg_send![event, magnification] };
        // NSEventPhaseBegan = 1, Ended = 8, Cancelled = 16.
        let phase: usize = unsafe { msg_send![event, phase] };
        MAGNIFY.with(|m| {
            let mut m = m.borrow_mut();
            let gesture = m.entry(this as *const _ as usize).or_default();
            gesture.delta += delta as f32;
            gesture.began |= phase & 1 != 0;
            gesture.ended |= phase & (8 | 16) != 0;
        });
    }
    unsafe extern "C" {
        fn object_setClass(object: id, class: *const Class) -> *const Class;
    }
    /// Add magnification handling to this window only, retaining GPUI's native view behavior.
    pub fn install_magnify(w: &gpui::Window) {
        let Some(view) = view(w) else {
            return;
        };
        static REGISTER: Once = Once::new();
        REGISTER.call_once(|| unsafe {
            let mut decl = ClassDecl::new("DesertUIMagnifiableView", (*view).class()).unwrap();
            decl.add_method(
                sel!(magnifyWithEvent:),
                magnify as extern "C" fn(&Object, Sel, id),
            );
            decl.register();
        });
        unsafe {
            object_setClass(view, Class::get("DesertUIMagnifiableView").unwrap());
        }
    }
    pub fn take_magnify_gesture(w: &gpui::Window) -> super::MagnifyGesture {
        view(w)
            .and_then(|v| MAGNIFY.with(|m| m.borrow_mut().remove(&(v as usize))))
            .unwrap_or_default()
    }
    pub fn take_magnification(w: &gpui::Window) -> f32 {
        take_magnify_gesture(w).delta
    }
    extern "C" fn operations(_: &Object, _: Sel, _: id, _: usize) -> usize {
        1
    } // copy
    extern "C" fn ended(this: &Object, _: Sel, _: id, _: NSPoint, _: usize) {
        unsafe {
            let _: () = msg_send![this, release];
        }
    }
    extern "C" fn ignore_modifiers(_: &Object, _: Sel, _: id) -> BOOL {
        YES
    }
    /// Hand original file URLs to AppKit so a drag can leave the application.
    /// Only call from a native mouse-drag event, on the UI thread.
    pub fn start_file_drag(w: &gpui::Window, paths: &[PathBuf]) -> bool {
        let Some(view) = view(w) else {
            return false;
        };
        if paths.is_empty() {
            return false;
        }
        static REGISTER: Once = Once::new();
        REGISTER.call_once(|| {
            let mut decl = ClassDecl::new("DesertUIFileDragSource", class!(NSObject)).unwrap();
            decl.add_protocol(
                Protocol::get("NSDraggingSource").expect("AppKit dragging source protocol"),
            );
            unsafe {
                decl.add_method(
                    sel!(draggingSession:sourceOperationMaskForDraggingContext:),
                    operations as extern "C" fn(&Object, Sel, id, usize) -> usize,
                );
                decl.add_method(
                    sel!(draggingSession:endedAtPoint:operation:),
                    ended as extern "C" fn(&Object, Sel, id, NSPoint, usize),
                );
                decl.add_method(
                    sel!(ignoreModifierKeysForDraggingSession:),
                    ignore_modifiers as extern "C" fn(&Object, Sel, id) -> BOOL,
                );
            }
            decl.register();
        });
        unsafe {
            let app: id = msg_send![class!(NSApplication), sharedApplication];
            let event: id = msg_send![app, currentEvent];
            if event == nil {
                return false;
            }
            let location: NSPoint = msg_send![event, locationInWindow];
            let location: NSPoint = msg_send![view, convertPoint:location fromView:nil];
            let items: id = msg_send![class!(NSMutableArray), array];
            for path in paths {
                let string = NSString::alloc(nil).init_str(&path.to_string_lossy());
                let url: id = msg_send![class!(NSURL), fileURLWithPath:string];
                let item: id = msg_send![class!(NSDraggingItem), alloc];
                let item: id = msg_send![item, initWithPasteboardWriter:url];
                let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
                let image: id = msg_send![workspace, iconForFile:string];
                let frame = NSRect::new(
                    NSPoint::new(location.x - 24., location.y - 24.),
                    NSSize::new(48., 48.),
                );
                let _: () = msg_send![item, setDraggingFrame:frame contents:image];
                let _: () = msg_send![items, addObject:item];
                let _: () = msg_send![item, release];
                let _: () = msg_send![string, release];
            }
            let source: id = msg_send![Class::get("DesertUIFileDragSource").unwrap(), new];
            let session: id =
                msg_send![view, beginDraggingSessionWithItems:items event:event source:source];
            if session == nil {
                let _: () = msg_send![source, release];
            }
            session != nil
        }
    }
}
#[cfg(target_os = "macos")]
pub use mac::{install_magnify, start_file_drag, take_magnification, take_magnify_gesture};
#[cfg(not(target_os = "macos"))]
pub fn take_magnification(_: &gpui::Window) -> f32 {
    0.
}
#[cfg(not(target_os = "macos"))]
pub fn take_magnify_gesture(_: &gpui::Window) -> MagnifyGesture {
    MagnifyGesture::default()
}
/// Trackpad pinch input accumulated since the last poll, with gesture boundaries.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MagnifyGesture {
    pub delta: f32,
    /// Fingers touched down since the last poll.
    pub began: bool,
    /// Fingers lifted (or the gesture was cancelled) since the last poll.
    pub ended: bool,
}
