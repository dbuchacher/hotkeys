use crate::apps::*;

pub fn vscode(mode: u8) {
    match mode {
        1 => {
            KeyBind::new(LBRACE, 1).send(vec![d(LCTRL), d(LSHIFT),   d(LBRACE),   u(LBRACE), u(LSHIFT), u(LCTRL)]).spawn(); // fold
            KeyBind::new(RBRACE, 1).send(vec![d(LCTRL), d(LSHIFT),   d(RBRACE),   u(RBRACE), u(LSHIFT), u(LCTRL)]).spawn(); // unfold
            KeyBind::new(U,      1).send(vec![d(LCTRL), d(LSHIFT),   d(K),        u(K), u(LSHIFT), u(LCTRL)]).spawn(); // delete line
            KeyBind::new(H,      1).send(vec![d(LCTRL), d(B),        u(B),        u(LCTRL)])                      .spawn(); // show/hide side bar
            KeyBind::new(J,      1).send(vec![d(LCTRL), d(ZERO),     u(ZERO),     u(LCTRL)])                      .spawn(); // focus side bar
            KeyBind::new(K,      1).send(vec![d(LCTRL), d(GRAVE),    u(GRAVE),    u(LCTRL)])                      .spawn(); // focus terminal
            KeyBind::new(I,      1).send(vec![d(LCTRL), d(ONE),      u(ONE),      u(LCTRL)])                      .spawn(); // focus editor
        },
        2 => {
            KeyBind::new(L,      1).send(vec![d(LCTRL), d(PAGEDOWN), u(PAGEDOWN), u(LCTRL)])                 .spawn(); // next tab
            KeyBind::new(J,      1).send(vec![d(LCTRL), d(PAGEUP),   u(PAGEUP),   u(LCTRL)])                 .spawn(); // prev tab
        },
        _ => (),
    }
}