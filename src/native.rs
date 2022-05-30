use super::env::Env;
use super::evaluator::error::EvalError;
use super::evaluator::eval_expr;
use super::parser::{Atom, Expr, Lambda, List};
use crate::atom_num;
use crate::nil_atom;
use std::collections::LinkedList;

// Expects only list args
pub fn add(list: List, env: &Env) -> Result<Expr, EvalError> {
    let mut sum: i32 = 0;
    for exp in list {
        let res = match eval_expr(exp, env)? {
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

pub fn sub(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    // TODO: Check Nil termination
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;
    let res = match eval_expr(car, env)? {
        Expr::Atom(a) => a,
        _ => unimplemented!(),
    };
    let mut res = match *res {
        Atom::Num(x) => x,
        a => return Err(EvalError::TypeMismatch("number".to_string(), a)),
    };

    let next = list.pop_front().ok_or(EvalError::EmptyList)?;
    let next = match eval_expr(next, env)? {
        Expr::Atom(a) => a,
        _ => unimplemented!(),
    };
    match *next {
        Atom::Num(x) => res -= x,
        Atom::Nil => return Ok(Expr::Atom(Box::new(Atom::Num(-res)))),
        a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
    }

    for exp in list {
        let expr_res = eval_expr(exp, env)?;
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

pub fn mul(list: List, env: &Env) -> Result<Expr, EvalError> {
    let mut res: i32 = 1;
    for exp in list {
        let expr_res = eval_expr(exp, env)?;
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

pub fn div(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    // TODO: Check Nil termination
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;
    let res = match eval_expr(car, env)? {
        Expr::Atom(a) => a,
        _ => unimplemented!(),
    };
    let mut res = match *res {
        Atom::Num(x) => x,
        a => return Err(EvalError::TypeMismatch("number".to_string(), a)),
    };

    let next = list.pop_front().ok_or(EvalError::EmptyList)?;
    let atom = match eval_expr(next, env)? {
        Expr::Atom(a) => *a,
        _ => unimplemented!(),
    };
    match atom {
        Atom::Num(x) => res /= x,
        Atom::Nil => return Ok(atom_num!(1 / res)),
        a => return Err(EvalError::TypeMismatch("number, nil".to_string(), a)),
    }

    for exp in list {
        let atom_res = match eval_expr(exp, env)? {
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

// TODO: handle all possible parameter variants
pub fn define(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    let car = list.pop_front().ok_or(EvalError::EmptyList)?;
    if !list.is_empty() && list.len() > 2 {
        println!("{:?}", list);
        return Err(EvalError::WrongNumOfArgs(2, list.len()));
    }

    let sym = match car {
        Expr::Atom(a) => *a,
        _ => todo!("return better error"),
    };
    let sym = match sym {
        Atom::Symbol(str) => str,
        _ => return Err(EvalError::TypeMismatch("symbol".to_string(), sym)),
    };

    let cdr = list.pop_front().unwrap();
    Ok(match eval_expr(cdr, env)? {
        Expr::Atom(a) => {
            env.insert(sym.as_str(), Expr::Atom(a.clone()));
            Expr::Atom(a)
        }
        Expr::Lambda(l) => {
            env.insert(sym.as_str(), Expr::Lambda(l.clone()));
            Expr::Lambda(l)
        }
        Expr::List(l) => {
            env.insert(sym.as_str(), Expr::List(l.clone()));
            Expr::List(l)
        }
        Expr::Quote(q) => {
            env.insert(sym.as_str(), *q.clone());
            *q
        }
        _ => todo!(),
    })
    //Ok(Expr::Atom(Box::new(val)))
}

/* Set global variables */
// TODO: handle all possible parameter variants
pub fn set(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    let sym = list.pop_front().ok_or(EvalError::EmptyList)?;

    if !list.is_empty() && list.len() != 2 {
        println!("{:?}", list);
        return Err(EvalError::WrongNumOfArgs(2, list.len()));
    }

    // TODO: Shrink this
    let sym = match sym {
        Expr::Atom(a) => *a,
        _ => todo!("proper error report"),
    };
    let sym = match sym {
        Atom::Symbol(str) => str,
        a => return Err(EvalError::TypeMismatch("symbol".to_string(), a)),
    };
    ////

    if !env.contains_symbol(&sym) {
        return Err(EvalError::UndefinedSymbol(sym));
    }

    let expr = list.pop_front().unwrap();
    Ok(match eval_expr(expr, env)? {
        Expr::Atom(a) => {
            env.insert(sym.as_str(), Expr::Atom(a.clone()));
            Expr::Atom(a)
        }
        Expr::Lambda(l) => {
            env.insert(sym.as_str(), Expr::Lambda(l.clone()));
            Expr::Lambda(l)
        }
        Expr::List(l) => {
            env.insert(sym.as_str(), Expr::List(l.clone()));
            Expr::List(l)
        }
        Expr::Quote(q) => {
            env.insert(sym.as_str(), *q.clone());
            *q
        }
        _ => todo!("more types"),
    })
}

pub fn quote(mut list: List, _env: &Env) -> Result<Expr, EvalError> {
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

pub fn cons(mut list: List, env: &Env) -> Result<Expr, EvalError> {
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

// TODO: Handle all possible parameter variants
pub fn lambda(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    if list.len() < 3 {
        return Err(EvalError::WrongNumOfArgs(2, list.len() - 1));
    }
    match list.pop_back().unwrap() {
        Expr::Atom(a) => match *a {
            Atom::Nil => {}
            _ => return Err(EvalError::DottedList),
        },
        _ => return Err(EvalError::DottedList),
    }

    let lambda_env = Env::new(None);
    // TODO: This has to be a Rc or reference,
    // should not be a copy
    lambda_env.set_outer(env.clone());

    let formals = list.pop_front().unwrap();
    let mut args_list: Vec<String> = vec![];
    match formals {
        Expr::List(arglist) => {
            let mut atom_list = match as_atoms(*arglist) {
                Ok(l) => l,
                _ => unimplemented!(),
            };
            let last = atom_list.pop_back().unwrap();
            for elem in atom_list {
                match elem {
                    Atom::Symbol(s) => {
                        lambda_env.insert(s.as_str(), Expr::Atom(Box::new(Atom::Nil)));
                        args_list.push(s);
                    }
                    _ => unimplemented!(),
                }
            }
            match last {
                // Mixed argument list
                Atom::Symbol(s) => {
                    lambda_env.insert(s.as_str(), Expr::List(Box::new(List::new())));
                    args_list.push(s);
                }
                // Fixed argument list
                Atom::Nil => {}
                _ => return Err(EvalError::TypeMismatch("symbol, nil".to_string(), last)),
            }
        }
        // Variadic argument list
        Expr::Atom(atom) => {
            if let Atom::Symbol(s) = *atom {
                lambda_env.insert(s.as_str(), Expr::Atom(Box::new(Atom::Nil)));
                args_list.push(s);
            }
        }
        // TODO: Throw better error
        _ => return Err(EvalError::TypeMismatch("list, atom".to_string(), Atom::Nil)),
    }

    let lambda = Lambda {
        args_list,
        body: list,
        env: lambda_env,
    };

    Ok(Expr::Lambda(Box::new(lambda)))
}

fn as_atoms(list: List) -> Result<LinkedList<Atom>, usize> {
    let iter = list.iter().map(|expr| {
        if let Expr::Atom(a) = expr {
            (true, *a.clone())
        } else {
            (false, Atom::Nil)
        }
    });

    let (bools, atoms): (Vec<bool>, LinkedList<Atom>) = iter.unzip();
    let error = bools.iter().position(|b| !b);
    if let Some(err) = error {
        return Err(err);
    }
    Ok(atoms)
}

pub fn inspect(_: List, env: &Env) -> Result<Expr, EvalError> {
    println!("{:?}", env);
    Ok(nil_atom!())
}
