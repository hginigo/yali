use crate::parser::{Atom, Expr};

pub fn expr_is_nil(expr: &Expr) -> bool {
    if let Expr::Atom(a) = expr {
        if let Atom::Nil = **a {
            true
        } else {
            false
        }
    } else {
        false
    }
}
