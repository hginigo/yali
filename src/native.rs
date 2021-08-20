use super::parser::{Expr, Atom, List};
use super::env::Env;
use crate::atom_num;
use crate::nil_atom;
// use super::parser::error::ParserErr;
use super::evaluator::eval_expr;
use super::evaluator::error::EvalError;

impl Default for Env {
    fn default() -> Self {
        Env::new(None)
    }
}

// Expects only list args
pub fn add(list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    let mut sum: i32 = 0;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    for exp in list {
        let res = match eval_expr(exp, &env)? {
            Expr::Atom(a) => a,
            _ => unimplemented!(),
        };
        let x = match *res {
            Atom::Num(x) => x,
             // This has to be the last element on the list
            Atom::Nil => break,
            a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
        };

        sum += x;
    }
    Ok(atom_num!(sum))
}

pub fn sub(mut list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    // TODO: Check Nil termination
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    let res = match eval_expr(car, &env)? {
        Expr::Atom(a) => a,
        _ => unimplemented!(),
    };
    let mut res = match *res {
        Atom::Num(x) => x,
        a => return Err(EvalError::TypeMismatch("number".to_string(), a)),
    };

    let next = list.pop_front().ok_or(EvalError::EmptyList)?;
    let next = match eval_expr(next, &env)? {
        Expr::Atom(a) => a,
        _ => unimplemented!(),
    };
    match *next {
        Atom::Num(x) => res -= x,
        Atom::Nil => return Ok(Expr::Atom(Box::new(Atom::Num(-1 * res)))),
        a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
    }

    for exp in list {
        let expr_res = eval_expr(exp, &env)?;
        let atom_res = match expr_res {
            Expr::Atom(a) => a,
            _ => unimplemented!(),
        };
        let x = match *atom_res {
            Atom::Num(x) => x,
            Atom::Nil => break,
            a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
        };
        res -= x;
    }

    Ok(atom_num!(res))
}

pub fn mul(list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    let mut res: i32 = 1;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    for exp in list {
        let expr_res = eval_expr(exp, &env)?;
        let atom_res = match expr_res {
            Expr::Atom(a) => a,
            _ => unimplemented!(),
        };
        let x = match *atom_res {
            Atom::Num(x) => x,
            Atom::Nil => break,
            a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
        };
        res *= x;
    }

    Ok(atom_num!(res))
}

pub fn div(mut list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    // TODO: Check Nil termination
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    let res = match eval_expr(car, &env)? {
        Expr::Atom(a) => a,
        _ => unimplemented!(),
    };
    let mut res = match *res {
        Atom::Num(x) => x,
        a => return Err(EvalError::TypeMismatch("number".to_string(), a)),
    };

    let next = list.pop_front().ok_or(EvalError::EmptyList)?;
    let atom = match eval_expr(next, &env)? {
        Expr::Atom(a) => *a,
        _=> unimplemented!(),
    };
    match atom {
        Atom::Num(x) => res /= x,
        Atom::Nil => return Ok(atom_num!(1 / res)),
        a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
    }

    for exp in list {
        let atom_res = match eval_expr(exp, &env)? {
            Expr::Atom(a) => *a,
            _ => unimplemented!(),
        };
        let x = match atom_res {
            Atom::Num(x) => x,
            Atom::Nil => break,
            a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
        };
        res /= x;
    }

    Ok(atom_num!(res))
}

}
