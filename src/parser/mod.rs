/*
 * Lisp Grammar definition
 *
 * Expression = Compound
 *            | List
 *            | Atom
 *            | Value
 *            | quote Expression
 *            | quasiquote Expression
 *            | unquote Expression
 *            | Expression assoc Expression
 * 
 * Compound = Expression Compound
 *          | nil
 *
 * List = ( Compound )
 *
 * Value = ( let name Expression )
 *
 * Atom = String
 *      | Number
 *      | Bool
 *      | nil
 *
 */
use super::tokenizer::{Token, TokenType};
use std::boxed::Box;
use std::collections::LinkedList;

#[macro_use]
mod error;
use crate::parser::error::ParserErr;

// TODO: quote and assoc
// Evaluable
#[derive(Debug)]
pub enum Expr {
    Atom(Box<Atom>),
    List(Box<List>),
    Quote(Box<Expr>),
    Quasiquote(Box<Expr>),
    Unquote(Box<Expr>),
}

#[derive(Debug)]
pub enum Atom {
    Str(String),
    Num(i32),
    Bool(bool),
    Value(String),
    Nil,
}

// Linked list
pub type List = LinkedList<Expr>;

fn parse_str(t: &Token) -> Result<String, error::ParserErr> {
    // let t = tokens.pop().ok_or(token_not_found!("Token not found parsing str"))?;
    match t.ttype {
        TokenType::Str => Ok(t.value.clone()),
        _ => Err(token_not_found!("<string>")),
    }
}

fn parse_num(t: &Token) -> Result<i32, error::ParserErr> {
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

fn parse_bool(t: &Token) -> Result<bool, error::ParserErr> {
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

fn parse_atom(tokens: &mut Vec<Token>) -> Result<Atom, error::ParserErr> {
    let t = tokens.pop().ok_or(token_not_found!("Token not found parsing atom"))?;

    if t.ttype == TokenType::Str {
        let s = parse_str(&t)?;
        return Ok(Atom::Str(s))
    }

    let n = parse_num(&t);
    if n.is_ok() {
        return Ok(Atom::Num(n.unwrap()));
    }

    let b = parse_bool(&t);
    if b.is_ok() {
        return Ok(Atom::Bool(b.unwrap()));
    }
    // match b {
    //     Ok(val) => Ok(Atom::Bool(val)),
    //     Err(_) => Err(unexpected_token!("<atom>", t)),
    // }
    return Ok(Atom::Value(t.value.clone()));
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<List, error::ParserErr> {
    let t = tokens.pop().ok_or(token_not_found!("Token not found parsing list"))?;
    let mut list: LinkedList<Expr> = LinkedList::new();

    if t.ttype != TokenType::Opc {
        return Err(unexpected_token!("(", t));
    }
    
    loop {
        let next = match tokens.last() {
            Some(next) => next,
            // TODO: Give the position of the error
            None => return Err(token_not_found!("Unexpected EOF parsing list, ')' may be missing")),
        };

        match next.ttype {
            TokenType::Clc => {
                tokens.pop().unwrap();
                return Ok(list);
            },
            _ => {
                let res = parse_expr(tokens);
                if res.is_ok() {
                    list.push_back(res.unwrap());
                } else {
                    return Err(res.unwrap_err());
                }
            },
        }
    }

}

#[macro_export]
macro_rules! nil_atom {
    () => { Expr::Atom(Box::new(Atom::Nil)) }
}

pub fn parse_expr(tokens: &mut Vec<Token>) -> Result<Expr, error::ParserErr> {
    let t = match tokens.last() {
        Some(t) => t,
        None => return Ok(nil_atom!()),
    };

    let res = match t.ttype {

        TokenType::Opc => {
            let l = parse_list(tokens)?;
            if l.len() == 0 { nil_atom!() }
            else { Expr::List(Box::new(l)) }
        },

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
        },

        TokenType::Str |
        TokenType::Other => {
            let a = parse_atom(tokens)?;
            Expr::Atom(Box::new(a))
        },
        _ => return Err(unexpected_token!("(, ', <,>, `, <string>, <atom>, <value>", tokens.pop().unwrap())),
    };

    Ok(res)
}

pub fn parse(mut _tokens: Vec<Token>) -> Vec<Expr> {
    unimplemented!();
}

