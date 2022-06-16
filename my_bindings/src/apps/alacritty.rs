use crate::apps::*;

pub fn alacritty() {
    KeyBind::new(N, 1).func(|| {
        KeyBind::remove(N); // remove keybind so we can use it later
        nano()
    }).spawn();
}

