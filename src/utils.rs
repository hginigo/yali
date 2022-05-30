use crate::evaluator::eval;
use crate::native::*;
use crate::parser::{Atom, Expr, NativeEnc};
use std::collections::HashMap;

#[macro_export]
macro_rules! atom_nil {
    () => {
        Expr::Atom(Box::new(Atom::Nil))
    };
}

#[macro_export]
macro_rules! atom_num {
    ($a:expr) => {
        Expr::Atom(Box::new(Atom::Num($a)))
    };
}

#[macro_export]
macro_rules! atom_native {
    ($name:ident) => {
        Atom::Native(NativeEnc($name))
    };
}

#[macro_export]
macro_rules! expr_atom {
    ($atom:expr) => {
        Expr::Atom(Box::new($atom))
    };
}

macro_rules! insert_native {
    ($id:ident, $symbol:expr, $func:ident) => {
        $id.insert($symbol.to_string(), expr_atom!(atom_native!($func)));
    };
}

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

pub fn init_map(map: &mut HashMap<String, Expr>) {
    insert_native!(map, "+", add);
    insert_native!(map, "-", sub);
    insert_native!(map, "*", mul);
    insert_native!(map, "/", div);
    insert_native!(map, "set!", set);
    insert_native!(map, "define", define);
    insert_native!(map, "inspect", inspect);
    insert_native!(map, "eval", eval);
    insert_native!(map, "quote", quote);
    insert_native!(map, "cons", cons);
    insert_native!(map, "lambda", lambda);
    map.insert("nil".to_string(), atom_nil!());
}
