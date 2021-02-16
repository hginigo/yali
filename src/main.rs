use yali::tokenizer::*;

fn main() {
    let st = "(+ (1       223 adawda 3)";
    let v = tokenize(&st);
    println!("{:?}", v);

    let s = "(\"foo\" \"bar\"\n\"ba\nz\")";
    let v = tokenize(&s);
    println!("{:?}", v);

    let s = "(\"foo\"; \"bar\"\n\"ba\nz\")";
    let v = tokenize(&s);
    println!("{:?}", v);

    let s = "(\"foo\" '(asd, 1) \"bar\"\n\"ba\nz\")";
    let v = tokenize(&s);
    println!("{:?}", v);
}
