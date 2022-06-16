use evdev::{Key, InputEvent, EventType};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::Actions;

// allows access of gobal variable 'KEYBINDS' from when we are in the event hook
pub static KEYBINDS: Lazy<Mutex<Vec<KeyBind>>> = Lazy::new(|| Mutex::new(vec![]));

// things that can happen when a KeyBind is activated
#[derive(Debug, Clone)]
pub enum Action {
    None,
    Send,
    Func,
}
// different type of options a KeyBind can contain
#[derive(Debug, Clone)]
pub struct KeyBind {
    pub trigger: Key,
    pub modifiers: Vec<Key>,
    pub action: Action,
    pub state: Option<i32>,
    pub unblock: bool,
    pub enable_modifiers: bool,
    pub func: Option<fn ()>,
    pub send: Option<Vec<InputEvent>>,
}

impl KeyBind {
    // default values for new KEYBINDS
    pub fn new(key: Key, state: i32) -> KeyBind {
        KeyBind {
            trigger: key,
            modifiers: Vec::new(),
            action: Action::None,
            state: Some(state),
            unblock: false,
            enable_modifiers: false,
            func: None,
            send: None,
        }
    }
    // without spawn the KeyBind won't be active
    pub fn spawn(self) {
        KEYBINDS.lock().unwrap().push(self);
    }
    // delete a KeyBind
    pub fn remove(key: Key) {
        KEYBINDS.lock().unwrap().retain(|x| x.trigger != key);
    }
    // delete all KEYBINDS
    pub fn remove_all() {
        KEYBINDS.lock().unwrap().clear();
    }
    // add modifier keys that need to be pressed before the KeyBind is pressed
    pub fn mods(mut self, key: Vec<Key>) -> Self {
        self.enable_modifiers = true;
        self.modifiers = key;
        self
    }
    // unblock the KeyBind key from also being sent
    pub fn unblock(mut self) -> Self {
        self.unblock = true;
        self
    }
    // fffffffffffffffffffffffffffffffffffff
    pub fn send(mut self, events: Vec<InputEvent>) -> Self {
        self.action = Action::Send;
        self.send = Some(events);
        self
    }
    // run code from an external function
    pub fn func(mut self, func: fn ()) -> Self {
        self.action = Action::Func;
        self.func = Some(func);
        self
    }
}

// makes life ez
pub struct SetKeyState;

impl SetKeyState {
    pub fn up(key: Key) { if key.state() { key.send(0) } }
    pub fn down(key: Key) { if !key.state() { key.send(1) } }
    pub fn switch(key: Key) { if key.state() { key.send(0) } if !key.state() { key.send(1) } }
}

pub fn remap(input: Key, output: Key ) {
    KeyBind::new(input, 1).send(vec![d(output)]).spawn();
    KeyBind::new(input, 0).send(vec![u(output)]).spawn();
}

pub fn d(key: Key) -> InputEvent {
    InputEvent::new(EventType::KEY, key.0, 1)
}

pub fn u(key: Key) -> InputEvent {
    InputEvent::new(EventType::KEY, key.0, 0)
}