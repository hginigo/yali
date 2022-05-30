use crate::atom_num;
use crate::env::Env;
use crate::evaluator::eval_expr;
use crate::parser::{parse, Atom, Expr};
use crate::tokenizer::tokenize;
use crate::utils::init_map;
use std::collections::HashMap;

#[test]
fn test() {
    let mut map: HashMap<String, Expr> = HashMap::default();
    init_map(&mut map);
    let env = Env::from(map);
    let res = eval_expr(parse(&mut tokenize("(+ 1 2)")).unwrap(), &env).unwrap();
    assert_eq!(res, atom_num!(3));
}
