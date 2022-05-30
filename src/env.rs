use super::parser::Expr;
use error::UndefinedSymbol;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub mod error {
    #[derive(Debug)]
    pub struct UndefinedSymbol {}
}

pub struct EnvType {
    pub symbols: HashMap<String, Expr>,
    pub outer: Option<Env>,
}

impl fmt::Debug for EnvType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "symbols: {:?}, outer: {}",
            self.symbols,
            if self.outer.is_some() {
                "{...}"
            } else {
                "None"
            }
        )
    }
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
            symbols: HashMap::default(),
            outer,
        })))
    }

    pub fn from(symbols: HashMap<String, Expr>) -> Env {
        Env(Rc::new(RefCell::new(EnvType {
            symbols,
            outer: None,
        })))
    }

    pub fn insert(&self, key: &str, val: Expr) {
        (*self.0)
            .borrow_mut()
            .symbols
            .insert(String::from(key), val);
    }

    pub fn set(&self, key: &str, val: Expr) {
        let env = self;

        // search for the key in our list of maps
        while !(*env.0).borrow().symbols.contains_key(key) {
            let _env = match env.0.borrow().outer.as_ref() {
                Some(env) => env,

                // if we reach the end without finding it,
                // insert the value in the first map
                None => {
                    self.insert(key, val);
                    return;
                }
            };
        }

        // once we find it overwrite
        env.insert(key, val);
    }

    // Read only
    fn find_env(&self, key: &str) -> Option<Env> {
        let env = (*self.0).borrow();
        if env.symbols.contains_key(key) {
            Some(self.clone())
        } else {
            match &env.outer {
                Some(e) => e.find_env(key),
                _ => None,
            }
        }
    }

    pub fn contains_symbol(&self, key: &str) -> bool {
        match self.find_env(key) {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn get(&self, key: &str) -> Result<Expr, UndefinedSymbol> {
        match self.find_env(key) {
            Some(env) => Ok((*env.0).borrow().symbols.get(key).unwrap().clone()),
            None => Err(UndefinedSymbol {}),
        }
    }

    pub fn set_outer(&self, outer: Env) {
        let s = &*self.0;
        s.borrow_mut().outer = Some(outer);
    }
}
