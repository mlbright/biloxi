use std::fs::File;
use std::io::prelude::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: biloxi [script]");
    } else if args.len() == 2 {
        run_file(&std::path::Path::new(&args[1]));
    } else {
        run_prompt();
    }
}

fn run_file(file: &std::path::Path) {
    let mut file = File::open(file).expect("Unable to open the program file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the program file");
    run(&contents);
}

fn run_prompt() {
    println!("repl!");
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn run(program: &str) {
    println!("program: {}", program);
}
