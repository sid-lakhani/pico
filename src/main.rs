use std::{ptr, process};
use std::io::Write;
use std::process::{Command, Stdio};
use x11::xlib;

// ---------------- Utility Formatters ----------------

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn rgb_to_rgb(r: u8, g: u8, b: u8) -> String {
    format!("rgb({}, {}, {})", r, g, b)
}

fn rgb_to_rgba(r: u8, g: u8, b: u8) -> String {
    format!("rgba({}, {}, {}, 1.0)", r, g, b)
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> String {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let l = (max + min) / 2.0;

    let (h, s);

    if max == min {
        h = 0.0;
        s = 0.0;
    } else {
        let d = max - min;
        s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

        h = if max == rf {
            (gf - bf) / d + if gf < bf { 6.0 } else { 0.0 }
        } else if max == gf {
            (bf - rf) / d + 2.0
        } else {
            (rf - gf) / d + 4.0
        } / 6.0;
    }

    format!(
        "hsl({:.0}, {:.0}%, {:.0}%)",
        h * 360.0,
        s * 100.0,
        l * 100.0
    )
}

// ---------------- Clipboard ----------------

fn copy_to_clipboard(text: &str) {
    if let Ok(mut child) = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
    }
}

// ---------------- Pixel sampling ----------------

unsafe fn pick_once() -> (u8, u8, u8) {
    let display = xlib::XOpenDisplay(ptr::null());
    if display.is_null() {
        eprintln!("pico: failed to open X display");
        process::exit(1);
    }

    let screen = xlib::XDefaultScreen(display);
    let root = xlib::XRootWindow(display, screen);

    // Change cursor → crosshair
    let cursor = xlib::XCreateFontCursor(display, 34); // XC_crosshair = 34
    xlib::XDefineCursor(display, root, cursor);

    // Grab mouse click
    let mask: u32 = (xlib::ButtonPressMask).try_into().unwrap();

    let grab = xlib::XGrabPointer(
        display,
        root,
        xlib::True,
        mask,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
        0,
        cursor,
        xlib::CurrentTime,
    );

    if grab != xlib::GrabSuccess {
        eprintln!("pico: could not grab pointer");
        process::exit(1);
    }

    let mut event: xlib::XEvent = std::mem::zeroed();

    loop {
        xlib::XNextEvent(display, &mut event);

        if event.get_type() == xlib::ButtonPress {
            let x = event.button.x_root;
            let y = event.button.y_root;

            let img = xlib::XGetImage(
                display,
                root,
                x,
                y,
                1,
                1,
                !0,
                xlib::ZPixmap,
            );

            let pixel = xlib::XGetPixel(img, 0, 0);

            xlib::XDestroyImage(img);
            xlib::XUngrabPointer(display, xlib::CurrentTime);
            xlib::XCloseDisplay(display);

            return (
                ((pixel >> 16) & 0xFF) as u8,
                ((pixel >> 8) & 0xFF) as u8,
                (pixel & 0xFF) as u8,
            );
        }
    }
}

// ---------------- Entry ----------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--h".to_string()) || args.contains(&"-h".to_string()) {
        println!(
"pico — minimal X11 color picker

Usage:
  pico               → HEX
  pico --rgb         → rgb(...)
  pico --rgba        → rgba(...,1.0)
  pico --hsl         → hsl(...)
  pico-rgb           (symlink)
  pico-rgba
  pico-hsl

Click anywhere to pick a color.
Auto-copies to clipboard.
Cursor becomes crosshair."
        );
        return;
    }

    unsafe {
        let (r, g, b) = pick_once();
        let exe = args[0].clone();

        let output = if exe.ends_with("pico-rgba") {
            rgb_to_rgba(r, g, b)
        } else if exe.ends_with("pico-hsl") {
            rgb_to_hsl(r, g, b)
        } else if args.contains(&"--rgba".to_string()) {
            rgb_to_rgba(r, g, b)
        } else if args.contains(&"--hsl".to_string()) {
            rgb_to_hsl(r, g, b)
        } else if args.contains(&"--rgb".to_string()) {
            rgb_to_rgb(r, g, b)
        } else {
            rgb_to_hex(r, g, b)
        };

        let block = format!("\x1b[48;2;{};{};{}m    \x1b[0m", r, g, b);

        println!("{}  {}", block, output);

        copy_to_clipboard(&output);
    }
}
