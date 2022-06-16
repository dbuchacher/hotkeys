use my_bindings::*;
use std::thread::sleep;


fn main() {

    //regular();


    fuck();

    KeyBind::new(PAUSE, 1).func(failsafe).spawn(); // exit

    pub fn failsafe() {
        KeyBind::new(ESC, 1).func(|| process::exit(1)).spawn();
    }

    if let Err(e) = set_hook() {
        eprintln!("Error setting hook: {}", e);
        process::exit(1);
    }
}