use yali::tokenizer::*;
use yali::parser::*;
use yali::env::*;
use yali::native::*;
use yali::evaluator::eval_expr;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::collections::HashMap;

fn repl() -> i32 {
    let mut rl = Editor::<()>::new();
    let mut initial_env = HashMap::default();
    initial_env.insert("+".to_string(), Expr::Atom(Box::new(Atom::Native(NativeEnc(add)))));
    initial_env.insert("-".to_string(), Expr::Atom(Box::new(Atom::Native(NativeEnc(sub)))));
    initial_env.insert("*".to_string(), Expr::Atom(Box::new(Atom::Native(NativeEnc(mul)))));
    initial_env.insert("/".to_string(), Expr::Atom(Box::new(Atom::Native(NativeEnc(div)))));
    let env = Env::from(initial_env);
    let exit_code;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut tokens = tokenize(line.as_str());
                let exprs = match parse(&mut tokens) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Parse err: {:?}", e);
                        continue
                    },
                };
                let ev = eval_expr(exprs, &env).unwrap_or(Atom::Nil);
                println!("{:?}", ev);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Interrupt");
                // exit_code = 1;
                // break
            },
            Err(ReadlineError::Eof) => {
                exit_code = 0;
                break
            },
            Err(e) => {
                println!("Error: {:?}", e);
                exit_code = 1;
                break
            }
        }
    }
    exit_code
}

fn main() {
    let res = repl();
    std::process::exit(res)
    // let s = "( 12\"foo\" '(asd, 1) \"bar\"\n.\"ba\nz\")";

    // println!("{}", s);
    // let mut v = tokenize(&s);
    // // println!("{:?}", v);

    // let p = parse(&mut v);
    // println!("{:?}", p);
}
