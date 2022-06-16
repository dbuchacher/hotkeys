use crate::apps::*;

pub fn firefox() {
    KeyBind::new(J,     1).send(vec![d(LCTRL), d(PAGEUP),   u(PAGEUP),   u(LCTRL)]).spawn(); // back a tab
    KeyBind::new(L,     1).send(vec![d(LCTRL), d(PAGEDOWN), u(PAGEDOWN), u(LCTRL)]).spawn(); // forward a tab
    KeyBind::new(N,     1).send(vec![d(LCTRL), d(T),        u(T),        u(LCTRL)]).spawn(); // new tab
    KeyBind::new(K,     1).send(vec![d(LCTRL), d(LBRACE),   u(LBRACE),   u(LCTRL)]).spawn(); // back a page
    KeyBind::new(I,     1).send(vec![d(LCTRL), d(RBRACE),   u(RBRACE),   u(LCTRL)]).spawn(); // forward a page
    KeyBind::new(SLASH, 1).send(vec![d(LCTRL), d(K),        u(K),        u(LCTRL)]).spawn(); // go to search bar
    KeyBind::new(O,     1).send(vec![d(LALT),  d(DOWN),     u(DOWN),     u(LALT)]) .spawn(); // forward search engine (in bar)
    KeyBind::new(U,     1).send(vec![d(LALT),  d(UP),       u(UP),       u(LALT)]) .spawn(); // back search engine (in bar)
}
