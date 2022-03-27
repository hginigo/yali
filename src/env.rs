use super::parser::Expr;
use error::UndefinedSymbol;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub mod error {
    #[derive(Debug)]
    pub struct UndefinedSymbol {}
}

#[derive(Debug)]
pub struct EnvType {
    pub symbols: RefCell<HashMap<String, Expr>>,
    pub outer: Option<Env>,
}

#[derive(Debug, Clone)]
pub struct Env(Rc<RefCell<EnvType>>);

impl std::cmp::PartialEq for Env {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new(None)
    }
}

impl Env {
    pub fn new(outer: Option<Env>) -> Env {
        Env(Rc::new(RefCell::new(EnvType {
            symbols: RefCell::new(HashMap::default()),
            outer: outer,
        })))
    }

    pub fn from(symbols: HashMap<String, Expr>) -> Env {
        Env(Rc::new(RefCell::new(EnvType {
            symbols: RefCell::new(symbols),
            outer: None,
        })))
    }

    pub fn insert(&self, key: &str, val: Expr) {
        (*self.0)
            .borrow_mut()
            .symbols
            .borrow_mut()
            .insert(String::from(key), val);
    }

    pub fn find_env(&self, key: &str) -> Option<Env> {
        if (*self.0).borrow().symbols.borrow().contains_key(key) {
            Some(self.clone())
        } else {
            match (*self.0).borrow().outer.clone() {
                Some(o) => o.find_env(key),
                _ => None,
            }
        }
    }

    pub fn get(&self, key: &str) -> Result<Expr, UndefinedSymbol> {
        match self.find_env(key) {
            Some(env) => Ok((*env.0).borrow().symbols.borrow().get(key).unwrap().clone()),
            None => Err(UndefinedSymbol {}),
        }
    }

    pub fn set_outer(&self, outer: Env) {
        let mut s = (*self.0).borrow_mut();
        s.outer = Some(outer);
    }
}
