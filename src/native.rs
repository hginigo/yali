use super::parser::{Atom, List};
use super::env::Env;
// use super::parser::error::ParserErr;
use super::evaluator::eval_expr;
use super::evaluator::error::EvalError;

impl Default for Env {
    fn default() -> Self {
        Env::new(None)
    }
}

// Expects only list args
pub fn add(list: List, env: Option<&Env>) -> Result<Atom, EvalError> {
    let mut sum: i32 = 0;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    for exp in list {
        let res = eval_expr(exp, &env)?;
        let x = match res {
            Atom::Num(x) => x,
             // This has to be the last element on the list
            Atom::Nil => break,
            _ => return Err(EvalError {}),
        };

        sum += x;
    }
    Ok(Atom::Num(sum))
}

pub fn sub(mut list: List, env: Option<&Env>) -> Result<Atom, EvalError> {
    // TODO: Check Nil termination
    let car = list.pop_front().ok_or(EvalError {})?;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    let mut res = match eval_expr(car, &env)? {
        Atom::Num(x) => x,
        _ => return Err(EvalError {}),
    };

    let next = list.pop_front().ok_or(EvalError {})?;
    match eval_expr(next, &env)? {
        Atom::Num(x) => res -= x,
        Atom::Nil => return Ok(Atom::Num(-1 * res)),
        _ => return Err(EvalError {}),
    }

    for exp in list {
        let atom_res = eval_expr(exp, &env)?;
        let x = match atom_res {
            Atom::Num(x) => x,
            Atom::Nil => break,
            _ => return Err(EvalError {}),
        };
        res -= x;
    }

    Ok(Atom::Num(res))
}

pub fn mul(list: List, env: Option<&Env>) -> Result<Atom, EvalError> {
    let mut res: i32 = 1;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    for exp in list {
        let atom_res = eval_expr(exp, &env)?;
        let x = match atom_res {
            Atom::Num(x) => x,
            Atom::Nil => break,
            _ => return Err(EvalError {}),
        };
        res *= x;
    }

    Ok(Atom::Num(res))
}

pub fn div(mut list: List, env: Option<&Env>) -> Result<Atom, EvalError> {
    // TODO: Check Nil termination
    let car = list.pop_front().ok_or(EvalError {})?;
    let env = env.map(|e| e.clone()).unwrap_or_default();
    let mut res = match eval_expr(car, &env)? {
        Atom::Num(x) => x,
        _ => return Err(EvalError {}),
    };

    let next = list.pop_front().ok_or(EvalError {})?;
    match eval_expr(next, &env)? {
        Atom::Num(x) => res /= x,
        Atom::Nil => return Ok(Atom::Num(1 / res)),
        _ => return Err(EvalError {}),
    }

    for exp in list {
        let atom_res = eval_expr(exp, &env)?;
        let x = match atom_res {
            Atom::Num(x) => x,
            Atom::Nil => break,
            _ => return Err(EvalError {}),
        };
        res /= x;
    }

    Ok(Atom::Num(res))
}
