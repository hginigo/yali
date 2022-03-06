use super::parser::{Expr, Atom, List};
use super::env::Env;
use crate::atom_num;
use crate::nil_atom;
// use super::parser::error::ParserErr;
use super::evaluator::error::EvalError;
use super::evaluator::eval_expr;

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

pub fn define(mut list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;
    if !list.is_empty() && list.len() > 2 {
        println!("{:?}", list);
        return Err(EvalError::WrongNumOfArgs(2, list.len()));
    }

    let e;
    let env = if env.is_some() {
        env.unwrap()
    } else {
        e = Env::new(None);
        &e
    };

    let sym = match car {
        Expr::Atom(a) => *a,
        _ => unimplemented!(),
    };
    let sym = match sym {
        Atom::Symbol(str) => str,
        _ => return Err(EvalError::TypeMismatch("symbol".to_string(), sym)),
    };

    let cdr = list.pop_front().unwrap();
    let val = match eval_expr(cdr, env)? {
        Expr::Atom(a) => *a,
        _ => unimplemented!(),
    };
    env.insert(&sym.as_str(), Expr::Atom(Box::new(val.clone())));
    Ok(Expr::Atom(Box::new(val)))
}

/* Set global variables */
pub fn set(mut list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;

    if !list.is_empty() && list.len() > 2 {
        println!("{:?}", list);
        return Err(EvalError::WrongNumOfArgs(2, list.len()));
    }

    let e;
    let env = if env.is_some() {
        env.unwrap()
    } else {
        e = Env::new(None);
        &e
    };
    let sym = match eval_expr(car, &env)? {
        Expr::Atom(a) => *a,
        _ => unimplemented!(),
    };
    let sym = match sym {
        Atom::Symbol(str) => str,
        a => return Err(EvalError::TypeMismatch("symbol".to_string(), a)),
    };

    let cdr = list.pop_front().unwrap();
    let val = match eval_expr(cdr, &env)? {
        Expr::Atom(a) => *a,
        _=> unimplemented!(),
    };
    env.insert(&sym.as_str(), Expr::Atom(Box::new(val.clone())));
    Ok(Expr::Atom(Box::new(val)))
}

pub fn quote(mut list: List, _env: Option<&Env>) -> Result<Expr, EvalError> {
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
        Ok(list.pop_front().unwrap())
    }
}

pub fn cons(mut list: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    let e;
    let env = if env.is_some() {
        env.unwrap()
    } else {
        e = Env::default();
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

    if list.len() != 2 {
        Err(EvalError::WrongNumOfArgs(2, list.len()))
    } else {
        let car = eval_expr(list.pop_front().unwrap(), env)?;
        let cdr = eval_expr(list.pop_back().unwrap(), env)?;
        let cons = if let Expr::List(mut l) = cdr {
            l.push_front(car);
            *l
        } else {
            let mut l = List::new();
            l.push_front(car);
            l.push_back(cdr);
            l
        };
        Ok(Expr::List(Box::new(cons)))
    }
}

pub fn inspect(_: List, env: Option<&Env>) -> Result<Expr, EvalError> {
    assert!(env.is_some());
    println!("{:?}", env.unwrap());
    Ok(nil_atom!())
}
