use crate::apps::*;

pub fn nano() {
    KeyBind::new(M,     1).send(vec![d(LCTRL), d(SIX), u(SIX), u(LCTRL)]).spawn(); // set mark text
    KeyBind::new(K,     1).send(vec![d(LALT),  d(SIX), u(SIX), u(LALT)]) .spawn(); // copy
    KeyBind::new(I,     1).send(vec![d(LCTRL), d(U),   u(U),   u(LCTRL)]).spawn(); // paste
    KeyBind::new(U,     1).send(vec![d(LCTRL), d(K),   u(K),   u(LCTRL)]).spawn(); // delete a line
    KeyBind::new(J,     1).send(vec![d(LALT),  d(U),   u(U),   u(LALT)]) .spawn(); // undo
    KeyBind::new(L,     1).send(vec![d(LALT),  d(E),   u(E),   u(LALT)]) .spawn(); // redo
    KeyBind::new(SLASH, 1).send(vec![d(LCTRL), d(W),   u(W),   u(LCTRL)]).spawn(); // find
    KeyBind::new(N,     1).send(vec![d(LCTRL), d(W),   u(W),   u(LCTRL)]).spawn(); // find-next
    KeyBind::new(P,     1).send(vec![d(LCTRL), d(Q),   u(Q),   u(LCTRL)]).spawn(); // find-prev
}