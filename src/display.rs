use super::parser::{Atom, Expr, PrintableList};
use crate::utils::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Atom::Str(s) => s.clone(),
            Atom::Num(n) => n.to_string(),
            Atom::Bool(b) => {
                if *b {
                    "#t".to_string()
                } else {
                    "#f".to_string()
                }
            }
            Atom::Symbol(s) => s.clone(),
            Atom::Nil => "()".to_string(),
            Atom::Native(_) => "<native>".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Expr::Atom(a) => format!("{}", a),
            Expr::List(l) => format!("{}", PrintableList(l.clone())),
            Expr::Quote(q) => format!("'{}", q),
            Expr::Quasiquote(q) => format!("`{}", q),
            Expr::Unquote(u) => format!(",{}", u),
            Expr::Lambda(l) => format!("<lambda>\n{:?}", l),
        };
        write!(f, "{}", s)
    }
}

impl Display for PrintableList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let list = &self.0;
        let mut s = String::from("(");
        let last = list.back().unwrap();

        let mut i = 1;
        for expr in list.iter() {
            let lag = format!("{}", expr);
            s.push_str(&lag);

            i += 1;
            if i == list.len() {
                break;
            }
            s.push(' ');
        }
        s.push_str(
            if expr_is_nil(&last) {
                "".to_string()
            } else {
                format!(" . {}", last)
            }
            .as_str(),
        );

        s.push(')');
        write!(f, "{}", s)
    }
}
