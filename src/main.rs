use yali::tokenizer::*;
use yali::parser::*;

fn main() {
    let s = "( 12\"foo\" '(asd, 1) \"bar\"\n\"ba\nz\")";

    println!("{}", s);
    let mut v = tokenize(&s);
    println!("{:?}", v);

    let p = parse_expr(&mut v);
    println!("{:?}", p);

}
