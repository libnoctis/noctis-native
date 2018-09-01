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

        self.content_over_titlebar = enabled
    }

    pub fn is_content_over_enabled(&self) -> bool {
        self.content_over_titlebar
    }

    pub fn set_title_displayed(&mut self, displayed: bool) {
        unsafe {
            self.ns_window().setTitleVisibility_(if displayed { NSWindowTitleVisibility::NSWindowTitleVisible } else { NSWindowTitleVisibility::NSWindowTitleHidden });
        }

        self.title_displayed = displayed;
    }

    pub fn is_title_displayed(&self) -> bool {
        self.title_displayed
    }

    pub fn set_titlebar_big(&mut self, big: bool) {
        unsafe {
            if big {
                let id = NSString::alloc(nil).init_str("titlebarStylingToolbar");
                let tb = NSToolbar::alloc(nil).initWithIdentifier_(id);

                tb.setShowsBaselineSeparator_(NO);
                self.ns_window().setToolbar_(tb);
            } else {
                self.ns_window().setToolbar_(nil);
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