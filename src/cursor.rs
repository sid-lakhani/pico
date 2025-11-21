use x11::xlib;
use std::ptr;

pub fn get_cursor_pos() -> (i32, i32) {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        let root = xlib::XDefaultRootWindow(display);

        let mut root_return = 0;
        let mut child_return = 0;
        let mut root_x = 0;
        let mut root_y = 0;
        let mut win_x = 0;
        let mut win_y = 0;
        let mut mask = 0;

        xlib::XQueryPointer(
            display,
            root,
            &mut root_return,
            &mut child_return,
            &mut root_x,
            &mut root_y,
            &mut win_x,
            &mut win_y,
            &mut mask,
        );

        (root_x, root_y)
    }
}

