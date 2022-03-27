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
        WrongNumOfArgs(i32, usize),
    }
}

pub fn eval_expr(exp: Expr, env: &Env) -> Result<Expr, EvalError> {
    match exp {
        Expr::Atom(box_atom) => eval_atom(*box_atom, env),
        Expr::List(list) => eval_list(*list, env),
        Expr::Quote(quo) => Ok(*quo),
        // TODO: match against all Expr types
        Expr::Lambda(lambda) => Ok(Expr::Lambda(lambda)),
        _ => unimplemented!(),
    }
}

pub fn eval_atom(atom: Atom, env: &Env) -> Result<Expr, EvalError> {
    match atom {
        Atom::Symbol(s) => {
            let exp = env.get(&s);
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
        // println!("{:?}", res);
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
        Expr::Atom(a) => {
            // println!("{:?}", a);
            match *a {
                Atom::Native(NativeEnc(f)) => Ok(f(list, Some(env))?),
                at => Ok(Expr::Atom(Box::new(at))),
            }
        }
        Expr::Lambda(l) => eval_lambda(*l, list, env),
        _ => unimplemented!(),
        //a => Err(EvalError::TypeMismatch("symbol".to_string(), a)),
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
            _ => panic!("TODO: Handle error"),
        }
    }
    // TODO: set outer to Some(Rc<Env>)
    lambda.env.set_outer(env.clone());

    // TODO: eval all the list
    eval_body(lambda.body, &lambda.env)
}

fn eval_body(body: List, env: &Env) -> Result<Expr, EvalError> {
    assert!(body.len() > 0, "body of the lambda cannot be empty.");
    let last = body.back().unwrap().clone();
    for expr in body {
        eval_expr(expr, env)?;
    }

    eval_expr(last, env)
}
