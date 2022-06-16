pub use linuxkeys::*;
pub use xdotool::window::{get_window_focus, get_window_name};

pub mod apps;
pub mod layouts;

pub use layouts::*;
pub use apps::*;

// make sure the keys aren't stuck down
fn release_keys() {
    for key in [
        ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, ZERO,
        MINUS, LBRACE, RBRACE, UP, DOWN, LEFT, RIGHT, LWIN, LCTRL, LALT,
        LSHIFT, HOME, END, PAGEUP, PAGEDOWN, INSERT, DELETE, TAB, RALT,
        F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
        // A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ] {
        key.send(0);
    }
}

// sleep timer for audio books
pub fn book_sleep() {
    SPACE.send(2);
    thread::sleep(std::time::Duration::from_secs(1*60*15));
    SPACE.send(2);
}

// check which window is active
pub fn focused_window_contains(app: &str) -> bool {
    // xdotool getwindowfocus getwindowname
    String::from_utf8(
        get_window_name(&String::from_utf8(
            get_window_focus("").stdout
        ).expect("Error with get_window_focus")).stdout
    ).expect("Error with get_window_name").contains(app)
}

// find a window name
pub fn find_window_name() {
    println!("{}", { 
        String::from_utf8(
            get_window_name(&String::from_utf8(
                get_window_focus("").stdout
            ).expect("Error with get_window_focus")).stdout
        ).expect("Error with get_window_name")
    });
}