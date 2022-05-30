use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use yali::env::*;
use yali::evaluator::eval_expr;
use yali::parser::error::ParserErr;
use yali::parser::*;
use yali::tokenizer::*;
use yali::utils::init_map;
use yali::*;

fn repl() -> i32 {
    let mut rl = Editor::<()>::new();

    let mut initial_env = HashMap::<String, Expr>::default();
    init_map(&mut initial_env);

    let env = Env::from(initial_env);
    let exit_code;

    let mut lines = String::new();
    let mut prompt = "> ";

    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                lines.push(' ');
                lines.push_str(&line);

                let mut tokens = tokenize(lines.as_str());
                let exprs = match parse(&mut tokens) {
                    Ok(e) => {
                        lines.clear();
                        prompt = "> ";
                        e
                    }
                    Err(e) => {
                        match e {
                            ParserErr::UnclosedList => {
                                prompt = "| ";
                            }
                            _ => {
                                println!("Parse error: {}", e);
                                lines.clear();
                                prompt = "> ";
                            }
                        };
                        continue;
                    }
                };
                let ev = match eval_expr(exprs, &env) {
                    Ok(a) => a,
                    a => {
                        println!("{:?}", a);
                        atom_nil!()
                    }
                };
                println!("{}", ev);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupt");
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
}
