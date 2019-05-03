use objc::runtime::{Object, Sel, objc_release};

use cocoa::base::{id, YES, NO, nil};
use cocoa::foundation::NSString;
use cocoa::appkit::{NSWindowStyleMask, NSWindowTitleVisibility, NSToolbar, NSWindow};

use winit::os::macos::WindowExt;
use winit::Window as WinitWindow;

pub struct Window {
    pub window: WinitWindow,

    // Titlebar
    pub title_displayed: bool,
    pub content_over_titlebar: bool,
    pub titlebar_big: bool
}

impl Window {
    pub fn enable_content_over(&mut self, enabled: bool) {
        if self.content_over_titlebar == enabled {
            return
        }

        unsafe {
            let ns_window = self.ns_window();
            let ns_view = self.window.get_nsview() as id;

            Window::apply_enable_content_over(enabled, ns_window, ns_view);
        }

        self.content_over_titlebar = enabled
    }

    unsafe fn apply_enable_content_over(enabled: bool, ns_window: id, ns_view: id) {
        let mut mask = ns_window.styleMask();

        if enabled {
            ns_window.setTitlebarAppearsTransparent_(YES);
            mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
        } else {
            ns_window.setTitlebarAppearsTransparent_(NO);
            mask &= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
        }

        ns_window.setStyleMask_(mask);
        ns_window.makeFirstResponder_(ns_view);
    }

    pub fn is_content_over_enabled(&self) -> bool {
        self.content_over_titlebar
    }

    pub fn set_title_displayed(&mut self, displayed: bool) {
        unsafe {
            self.ns_window().setTitleVisibility_(
                if displayed {
                    NSWindowTitleVisibility::NSWindowTitleVisible
                } else {
                    NSWindowTitleVisibility::NSWindowTitleHidden
                }
            );
        }

        self.title_displayed = displayed;
    }

    pub fn is_title_displayed(&self) -> bool {
        self.title_displayed
    }

    pub fn set_titlebar_big(&mut self, big: bool) {
        unsafe fn add_toolbar(window: id) {
            let id = NSString::alloc(nil).init_str("titlebarStylingToolbar");
            let tb = NSToolbar::alloc(nil).initWithIdentifier_(id);

            tb.setShowsBaselineSeparator_(NO);
            window.setToolbar_(tb);
        }

        // TODO: Show title on fullscreen ?

        unsafe {
            if big {
                add_toolbar(self.ns_window());

                extern fn on_enter_fullscreen(this: &Object, _cmd: Sel, _notification: id) {
                    unsafe {
                        let window: id = *this.get_ivar("window");
                        window.setToolbar_(nil);
                    }
                }

                extern fn on_did_enter_fullscreen(this: &Object, _cmd: Sel, _notification: id) {
                    unsafe {
                        let ns_window: id = *this.get_ivar("window");
                        let ns_view: id = *this.get_ivar("view");

                        add_toolbar(ns_window);

                        Window::apply_enable_content_over(true, ns_window, ns_view)
                    }
                }

                self.ns_window().setDelegate_(delegate!("NoctisWindowDelegate", {
                    window: id = self.ns_window(),
                    view: id = self.window.get_nsview() as id,

                    (windowWillEnterFullScreen:) => on_enter_fullscreen as extern fn(&Object, Sel, id),
                    (windowDidEnterFullScreen:) => on_did_enter_fullscreen as extern fn(&Object, Sel, id)
                }));
            } else {
                let window = self.ns_window();
                let tb = window.toolbar();

                if tb != nil {
                    window.setToolbar_(nil);
                    objc_release(tb);
                }
            }
        }

        self.titlebar_big = big
    }

    pub fn is_titlebar_big(&self) -> bool {
        self.titlebar_big
    }

    fn ns_window(&self) -> id {
        self.window.get_nswindow() as id
    }
}