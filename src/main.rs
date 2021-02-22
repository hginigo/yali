use yali::tokenizer::*;
use yali::parser::*;
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn repl() -> i32 {
    let mut rl = Editor::<()>::new();
    let exit_code;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut tokens = tokenize(line.as_str());
                let exprs = parse(&mut tokens);
                println!("{:?}", exprs);
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
