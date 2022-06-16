use crate::*;
// use evdev::{Device, InputEvent, EventType};

pub fn failsafe() {
    KeyBind::new(ESC, 1).func(|| process::exit(1)).spawn();
}

// used at the start of most functions
fn restore_regular(trigger: Key, remove_keybinds: bool) {
    release_keys();
    if remove_keybinds {
        KeyBind::remove_all();
    }
    KeyBind::new(trigger, 0).func(regular).spawn();
}

pub fn fuck() {


    // VD.lock().unwrap().emit(&[InputEvent::new(EventType::KEY, 55, 0)]).unwrap();

    // println!("{}", x);

    KeyBind::new(Q, 1).func(cunt).spawn();




}
pub fn cunt() {

    println!(":::");
    VD.lock().unwrap().emit(&[InputEvent::new(EventType::KEY, 97, 1)]).unwrap();
    T.send(2);
}

// regular keyboard layout used for normal typing
pub fn regular() {
    KeyBind::remove_all();
    println!("regular");


    KeyBind::new(PAUSE, 1).func(failsafe).spawn(); // exit


    KeyBind::new(CAPSLOCK,  1).func(movement).spawn();
    KeyBind::new(F1,        1).func(apps1)   .spawn();
    KeyBind::new(F2,        1).func(apps2)   .spawn();
    KeyBind::new(RALT,      1).func(common)  .spawn();
    KeyBind::new(TAB,       1).func(desktop) .spawn();
    KeyBind::new(BACKSLASH, 1).func(desktop) .spawn();
}

// besides regular this is used most often
pub fn movement() {
    restore_regular(CAPSLOCK, false);
    KeyBind::remove(BACKSLASH);
    KeyBind::new(F12, 1).func(|| process::exit(1)).spawn(); // exit
    KeyBind::new(F11, 1).func(audio_sleep)     .spawn();    // audio book
    KeyBind::new(F8,  1).func(find_window_name).spawn();    // win name
    KeyBind::new(F9,  1).func(game)            .spawn();    // game mode

    remap(RALT,      ESC);
    remap(A,         LCTRL);
    remap(S,         LALT); 
    remap(D,         LSHIFT);
    remap(I,         UP);
    remap(K,         DOWN);
    remap(J,         LEFT);
    remap(L,         RIGHT);
    remap(U,         BACKSPACE);
    remap(O,         DELETE);
    remap(H,         HOME);
    remap(SEMICOLON, END);
    remap(COMMA,     PAGEUP);
    remap(DOT,       PAGEDOWN);
    remap(SPACE,     TAB);
    remap(BACKSLASH, BACKSLASH);
}

// when tab is held down; program specific 
pub fn apps1() {
    restore_regular(F1, false);

         if focused_window_contains("Firefox")            { firefox()   }
    else if focused_window_contains("Visual Studio Code") { vscode(1)   } 
    else if focused_window_contains("Alacritty")          { alacritty() }
    else if focused_window_contains("root@:")             { alacritty() }
}

// when tab is held down; program specific 
pub fn apps2() {
    restore_regular(F2, false);

    if focused_window_contains("Visual Studio Code") { vscode(2) }
}

// things like undo and copy
pub fn common() {
    restore_regular(RALT, false);

    LCTRL.send(1);

    remap(J, Z); // undo
    remap(L, Y); // redo
    remap(K, C); // copy
    remap(I, V); // paste
}

// desktop enviroment stuff
pub fn desktop() {
    restore_regular(TAB,       false);
    restore_regular(BACKSLASH, false);

    LWIN.send(1);
    
    KeyBind::remove(CAPSLOCK);
    KeyBind::new(CAPSLOCK, 1).send(vec![d(CAPSLOCK), u(CAPSLOCK)]).spawn();
    remap(Q, LCTRL);
    remap(W, LALT);
    remap(E, LSHIFT);
}

// used at the start of game functions
fn restore_game(trigger: Key) {
    KeyBind::remove_all();
    KeyBind::new(trigger, 0).func(game).spawn();
}

// sleeper
fn audio_sleep() {
    println!("fdus");
    restore_regular(PAUSE, true);
    KeyBind::new(ENTER2, 1).func(|| {
        thread::spawn(|| book_sleep());
    }).spawn();
}

// games
pub fn game() {
    restore_regular(PAUSE, true); println!("game");

    remap(S, W);  // W
                  // A
    remap(X, S);  // S
                  // D

    KeyBind::new(F, 1).func(actions).spawn();
    KeyBind::new(Z, 1).func(low_nums).spawn();
    KeyBind::new(C, 1).func(high_nums).spawn();

    fn actions() {
        restore_game(F);
        E.send(2); 
        remap(LBUTTON, R);
    }   
    fn low_nums() {
        restore_game(Z);
        remap(LBUTTON, ONE);
        remap(RBUTTON, TWO);
        remap(BUTTON1, THREE);
        remap(BUTTON2, FOUR);
    }  
    fn high_nums() {
        restore_game(C);
        remap(LBUTTON, FIVE);
        remap(RBUTTON, SIX);
        remap(BUTTON1, SEVEN);
        remap(BUTTON2, EIGHT);
    }  
}