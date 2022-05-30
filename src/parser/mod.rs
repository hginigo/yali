/*
 * Lisp Grammar definition
 *
 * Expression = List
 *            | Atom
 *            | Value
 *            | quote Expression
 *            | quasiquote Expression
 *            | unquote Expression
 *            | Expression assoc Expression
 *
 * List = ( Compound )
 *
 * Compound = Expression Compound
 *          | nil
 *
 * Value = ( let name Expression )
 *
 * Atom = String
 *      | Number
 *      | Bool
 *      | nil
 *
 */
use super::env::Env;
use super::evaluator::error::EvalError;
use super::tokenizer::{Token, TokenType};
use crate::atom_nil;
use std::boxed::Box;
use std::collections::LinkedList;
use std::fmt;

#[macro_use]
pub mod error;
use error::ParserErr;

// TODO: quote and assoc
// Evaluable
#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Atom(Box<Atom>),
    List(Box<List>),
    Quote(Box<Expr>),
    Quasiquote(Box<Expr>),
    Unquote(Box<Expr>),
    Lambda(Box<Lambda>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Atom {
    Str(String),
    Num(i32),
    Bool(bool),
    Symbol(String),
    Nil,
    Native(NativeEnc),
}

// Linked list
pub type List = LinkedList<Expr>;
pub struct PrintableList(pub Box<List>);

#[derive(PartialEq, Debug, Clone)]
pub struct Lambda {
    pub args_list: Vec<String>,
    pub body: List,
    pub env: Env,
}

pub type NativeFn = fn(List, &Env) -> Result<Expr, EvalError>;
pub struct NativeEnc(pub NativeFn);

impl fmt::Debug for NativeEnc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native>")
    }
}

impl std::cmp::PartialEq for NativeEnc {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Clone for NativeEnc {
    fn clone(&self) -> Self {
        NativeEnc(self.0)
    }
}

fn parse_str(t: &Token) -> Result<String, ParserErr> {
    // let t = tokens.pop().ok_or(token_not_found!("Token not found parsing str"))?;
    match t.ttype {
        TokenType::Str => Ok(t.value.clone()),
        _ => Err(token_not_found!("<string>")),
    }
}

fn parse_num(t: &Token) -> Result<i32, ParserErr> {
    // let t = tokens.pop().ok_or(token_not_found!("Token not found parsing num"))?;

    if t.ttype != TokenType::Other {
        return Err(token_not_found!("<number>"));
    }

    let num = match t.value.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(error::ParserErr::ParseInt(e)),
    };

    Ok(num)
}

fn parse_bool(t: &Token) -> Result<bool, ParserErr> {
    if t.ttype != TokenType::Other {
        return Err(token_not_found!("<bool>"));
    };

    let val = if t.value == "#t" {
        true
    } else if t.value == "#f" {
        false
    } else {
        return Err(token_not_found!("<bool>"));
    };

    Ok(val)
}

fn parse_atom(tokens: &mut Vec<Token>) -> Result<Atom, ParserErr> {
    let t = tokens
        .pop()
        .ok_or_else(|| token_not_found!("Token not found parsing atom"))?;

    if t.ttype == TokenType::Str {
        let s = parse_str(&t)?;
        return Ok(Atom::Str(s));
    }

    let num = parse_num(&t);
    if let Ok(n) = num {
        return Ok(Atom::Num(n));
    }

    let bool = parse_bool(&t);
    if let Ok(b) = bool {
        return Ok(Atom::Bool(b));
    }
    // match b {
    //     Ok(val) => Ok(Atom::Bool(val)),
    //     Err(_) => Err(unexpected_token!("<atom>", t)),
    // }
    Ok(Atom::Symbol(t.value))
}

fn parse_cdr(tokens: &mut Vec<Token>) -> Result<Expr, ParserErr> {
    let t = tokens
        .pop()
        .ok_or_else(|| token_not_found!("Token not found parsing cons"))?;

    if t.ttype != TokenType::Dot {
        return Err(unexpected_token!(t, "."));
    }

    let expr = parse_expr(tokens)?;
    let end = tokens.pop().ok_or_else(|| unclosed_list!())?;

    match end.ttype {
        TokenType::Clc => Ok(expr),
        _ => Err(unexpected_token!(end, ")")),
    }
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<List, ParserErr> {
    let t = tokens
        .pop()
        .ok_or_else(|| token_not_found!("Token not found parsing list"))?;
    let mut list: LinkedList<Expr> = LinkedList::new();

    if t.ttype != TokenType::Opc {
        return Err(unexpected_token!(t, "("));
    }

    loop {
        let next = match tokens.last() {
            Some(next) => next,
            // TODO: Give the position of the error
            None => return Err(unclosed_list!()),
        };

        match next.ttype {
            TokenType::Dot => {
                if list.is_empty() {
                    return Err(unexpected_token!(
                        tokens.pop().unwrap(),
                        "(",
                        "'",
                        "<",
                        ">",
                        "`",
                        "<string>",
                        "<atom>",
                        "<value>"
                    ));
                }
                let cdr = parse_cdr(tokens)?;
                list.push_back(cdr);
                return Ok(list);
            }

            TokenType::Clc => {
                tokens.pop().unwrap();
                list.push_back(atom_nil!());
                return Ok(list);
            }

            _ => {
                let res = parse_expr(tokens);
                if let Ok(res) = res {
                    list.push_back(res);
                } else {
                    return Err(res.unwrap_err());
                }
            }
        }
    }
}

pub fn parse_expr(tokens: &mut Vec<Token>) -> Result<Expr, ParserErr> {
    let t = match tokens.last() {
        Some(t) => t,
        None => return Ok(atom_nil!()),
    };

    let res = match t.ttype {
        TokenType::Opc => {
            let l = parse_list(tokens)?;
            if l.len() <= 1 {
                atom_nil!()
            } else {
                Expr::List(Box::new(l))
            }
        }

        TokenType::Quo => {
            tokens.pop().unwrap();
            let q = parse_expr(tokens)?;
            Expr::Quote(Box::new(q))
        }
        TokenType::Unquo => {
            tokens.pop().unwrap();
            let q = parse_expr(tokens)?;
            Expr::Unquote(Box::new(q))
        }
        TokenType::Quasi => {
            tokens.pop().unwrap();
            let q = parse_expr(tokens)?;
            Expr::Quasiquote(Box::new(q))
        }

        TokenType::Str | TokenType::Other => {
            let a = parse_atom(tokens)?;
            Expr::Atom(Box::new(a))
        }

        _ => {
            return Err(unexpected_token!(
                tokens.pop().unwrap(),
                "(",
                "'",
                "<",
                ">",
                "`",
                "<string>",
                "<atom>",
                "<value>"
            ))
        }
    };

    Ok(res)
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Expr, ParserErr> {
    let expr = parse_expr(tokens)?;

    if tokens.is_empty() {
        Ok(expr)
    } else {
        Err(unexpected_token!(tokens.pop().unwrap(), "EOF"))
    }
}
