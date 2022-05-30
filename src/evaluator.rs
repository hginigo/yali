use super::env::Env;
use super::parser::{Atom, Expr, Lambda, List, NativeEnc};
use error::EvalError;
use std::convert::TryInto;

pub mod error {
    use crate::parser::{Atom, Expr};
    #[derive(Debug)]
    pub enum EvalError {
        DottedList,
        UndefinedSymbol(String),
        TypeMismatch(String, Atom),
        ExprTypeMismatch(String, Expr),
        EmptyList,
        WrongNumOfArgs(usize, usize),
    }
}

pub fn eval_expr(exp: Expr, env: &Env) -> Result<Expr, EvalError> {
    match exp {
        Expr::Atom(box_atom) => eval_atom(*box_atom, env),
        Expr::List(list) => eval_list(*list, env),
        Expr::Quote(quo) => Ok(*quo),
        // TODO: match against all Expr types
        Expr::Lambda(lambda) => Ok(Expr::Lambda(lambda)),
        _ => todo!("more types"),
    }
}

pub fn eval_atom(atom: Atom, env: &Env) -> Result<Expr, EvalError> {
    match atom {
        Atom::Symbol(s) => {
            let expr = env.get(&s);
            expr.map_err(|_| EvalError::UndefinedSymbol(s))
        }
        other => Ok(Expr::Atom(Box::new(other))),
    }
}

pub fn eval(mut list: List, env: &Env) -> Result<Expr, EvalError> {
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
        let res = eval_expr(list.pop_front().unwrap(), env)?;
        Ok(eval_expr(res, env)?)
    }
}

pub fn eval_list(mut list: List, env: &Env) -> Result<Expr, EvalError> {
    let first = list.pop_front().ok_or(EvalError::EmptyList)?;

    match eval_expr(first, env)? {
        Expr::Atom(a) => match *a {
            Atom::Native(NativeEnc(f)) => Ok(f(list, env)?),
            other => Ok(Expr::Atom(Box::new(other))),
        },
        Expr::Lambda(l) => eval_lambda(*l, list, env),
        _ => todo!(), //a => Err(EvalError::TypeMismatch("symbol".to_string(), a)),
    }
}

pub fn eval_lambda(lambda: Lambda, mut args: List, env: &Env) -> Result<Expr, EvalError> {
    args.pop_back().unwrap();
    let lambda_args_count = lambda.args_list.len();
    let args_count = args.len();

    if args_count != lambda_args_count {
        return Err(EvalError::WrongNumOfArgs(
            lambda_args_count.try_into().unwrap(),
            args_count,
        ));
    }

    for (val, syn) in args.iter().zip(lambda.args_list.iter()) {
        match val {
            Expr::Atom(a) => {
                // TODO: handle clone
                lambda.env.insert(syn, Expr::Atom(a.clone()));
            }
            _ => todo!(),
        }
    }
    // TODO: set outer to Some(Rc<Env>)
    lambda.env.set_outer(env.clone());

    // TODO: eval all the list
    eval_body(lambda.body, &lambda.env)
}

fn eval_body(body: List, env: &Env) -> Result<Expr, EvalError> {
    assert!(!body.is_empty(), "body of the lambda cannot be empty.");
    let last = body.back().unwrap().clone();
    for expr in body {
        eval_expr(expr, env)?;
    }

    eval_expr(last, env)
}
