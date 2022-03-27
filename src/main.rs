use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use yali::env::*;
use yali::evaluator::{eval, eval_expr};
use yali::native::*;
use yali::nil_atom;
use yali::parser::*;
use yali::tokenizer::*;

fn repl() -> i32 {
    let mut rl = Editor::<()>::new();
    let mut initial_env = HashMap::default();
    initial_env.insert(
        "+".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(add)))),
    );
    initial_env.insert(
        "-".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(sub)))),
    );
    initial_env.insert(
        "*".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(mul)))),
    );
    initial_env.insert(
        "/".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(div)))),
    );
    initial_env.insert(
        "set".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(set)))),
    );
    initial_env.insert(
        "define".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(define)))),
    );
    initial_env.insert(
        "inspect".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(inspect)))),
    );
    initial_env.insert(
        "eval".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(eval)))),
    );
    initial_env.insert(
        "quote".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(quote)))),
    );
    initial_env.insert(
        "cons".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(cons)))),
    );
    initial_env.insert(
        "lambda".to_string(),
        Expr::Atom(Box::new(Atom::Native(NativeEnc(lambda)))),
    );
    initial_env.insert("nil".to_string(), Expr::Atom(Box::new(Atom::Nil)));
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
                        continue;
                    }
                };
                let ev = match eval_expr(exprs, &env) {
                    Ok(a) => a,
                    a => {
                        println!("{:?}", a);
                        nil_atom!()
                    }
                };
                println!("{}", ev);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupt");
                // exit_code = 1;
                // break
            }
            Err(ReadlineError::Eof) => {
                exit_code = 0;
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                exit_code = 1;
                break;
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
