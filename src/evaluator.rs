use super::parser::{Atom, Expr, List, NativeEnc};
use super::env::Env;
use error::EvalError;
pub mod error {
    pub struct EvalError {
    }
}

pub fn eval_expr(exp: Expr, env: &Env) -> Result<Atom, EvalError> {
    //println!("{:?}", exp);
    match exp {
        Expr::Atom(box_atom) => {
            return Ok(*box_atom);
        }

        Expr::List(list) => {
            return Ok(eval_list(*list, env)?);
        }

        _ => unimplemented!(),
    }
}

pub fn eval_list(mut list: List, env: &Env) -> Result<Atom, EvalError> {
    let first = list.pop_front().ok_or(EvalError {})?;

    let op_atom = eval_expr(first, env)?;
    let op = match op_atom {
        Atom::Value(opr) => opr,
        _ => return Err(EvalError {}),
    };

    let res = match env.get(op.as_str()) {
        Ok(exp) => exp,
        _ => return Err(EvalError {}),
    };
    let res = eval_expr(res, env)?;
    Ok(match res {
        Atom::Native(NativeEnc(f)) => f(list, Some(env)),
        _ => Err(EvalError {}),
    }?)
    // Ok(match op.as_str() {
    //     "+" => add(list, None),
    //     "-" => sub(list, None),
    //     "*" => mul(list, None),
    //     "/" => div(list, None),
    //     _ => Err(EvalError {}),
    // }?)
}
