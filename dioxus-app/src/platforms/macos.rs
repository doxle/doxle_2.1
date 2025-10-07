use cocoa::appkit::NSApp;
use cocoa::base::{id, nil, YES};
use cocoa::foundation::NSUInteger;
use objc::{msg_send, sel, sel_impl};

use std::time::Duration;
use dispatch::Queue;

// Apply macOS unified titlebar (transparent + full-size content view).
// We schedule a few attempts on the main queue to handle cases where the window
// isn’t key yet on first mount.
pub fn setup_unified_titlebar() {
    let attempts = [0u64, 50, 150, 300];
    for ms in attempts {
        Queue::main().exec_after(Duration::from_millis(ms), || unsafe {
            let _ = try_setup_once();
        });
    }
}

unsafe fn try_setup_once() -> bool {
    let app = NSApp();
    if app == nil {
        return false;
    }

    // Prefer keyWindow; fall back to mainWindow if needed.
    let mut window: id = msg_send![app, keyWindow];
    if window == nil {
        window = msg_send![app, mainWindow];
        if window == nil {
            return false;
        }
    }

    // Make the titlebar transparent
    let _: () = msg_send![window, setTitlebarAppearsTransparent: YES];

    // Hide the window title text (optional for cleaner look)
    // NSWindowTitleHidden = 1
    let title_hidden: NSUInteger = 1;
    let _: () = msg_send![window, setTitleVisibility: title_hidden];

    // Enable full-size content view so content extends into the titlebar area
    // NSWindowStyleMaskFullSizeContentView = 1 << 15
    let mut style_mask: NSUInteger = msg_send![window, styleMask];
    let fullsize_mask: NSUInteger = 1 << 15;
    style_mask |= fullsize_mask;
    let _: () = msg_send![window, setStyleMask: style_mask];

    // Allow dragging by background (useful if you mark your nav as drag region)
    let _: () = msg_send![window, setMovableByWindowBackground: YES];

    true
}
