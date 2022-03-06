use super::parser::{Atom, Expr, List, NativeEnc};
use super::env::Env;
use error::EvalError;

pub mod error {
    use crate::parser::Atom;
    #[derive(Debug)]
    pub enum EvalError {
        DottedList,
        UndefinedSymbol(String),
        TypeMismatch(String, Atom),
        ExprTypeMismatch(String, Expr),
        EmptyList,
        WrongNumOfArgs(i32, usize),
    }
}

pub fn eval_expr(exp: Expr, env: &Env) -> Result<Expr, EvalError> {
    match exp {
        Expr::Atom(box_atom) => eval_atom(*box_atom, env),
        Expr::List(list) => eval_list(*list, env),
        Expr::Quote(quo) => Ok(*quo),
        _ => unimplemented!(),
    }
}

pub fn eval_atom(atom: Atom, env: &Env) -> Result<Expr, EvalError> {
    match atom {
        Atom::Symbol(s) => {
            let exp =  env.get(&s);
            if exp.is_ok() {
                eval_expr(exp.unwrap(), env)
            } else {
                Err(EvalError::UndefinedSymbol(s.clone()))
            }
        }
        a => Ok(Expr::Atom(Box::new(a))),
    }
}

pub fn eval(mut list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    let e;
    let env = if env.is_some() {
        env.unwrap()
    } else {
        e = Env::new(None);
        &e
    };
    match list.pop_back() {
        Some(Expr::Atom(nil)) => {
            if *nil != Atom::Nil {
                return Err(EvalError::DottedList);
            }
        }
        Some(_) => return Err(EvalError::DottedList),
        None => return Err(EvalError::EmptyList),
    };
    if list.len() != 1 {
        Err(EvalError::WrongNumOfArgs(1, list.len()))
    } else {
        let res = eval_expr(list.pop_front().unwrap(), &env)?;
        println!("{:?}", res);
        Ok(eval_expr(res, &env)?)
    }
}

pub fn eval_list(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    let first = list.pop_front().ok_or(EvalError::EmptyList)?;

    // println!("{:?}", first);
    // let op = match eval_expr(first, env)? {
    //     Atom::Value(opr) => opr,
    //     a => return Err(EvalError::TypeMismatch("symbol".to_string(), a)),
    // };

    // let res = match env.get(op.as_str()) {
    //     Ok(exp) => exp,
    //     _ => return Err(EvalError::UndefinedSymbol(op)),
    // };
    match eval_expr(first, env)? {
        Expr::Atom(a) => match *a {
            Atom::Native(NativeEnc(f)) => Ok(f(list, Some(env))?),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
        //a => Err(EvalError::TypeMismatch("symbol".to_string(), a)),
    }
}
