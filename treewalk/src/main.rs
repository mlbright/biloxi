use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::File;
use std::io::prelude::*;

mod scanner;
mod token;
mod token_type;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => run_file(&std::path::Path::new(&args[1])),
        3.. => {
            println!("Usage: biloxi [script]");
            std::process::exit(64);
        }
        _ => run_prompt(),
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
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                run(line.as_str());
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

fn run(source: &str) {
    println!("program: {}", source);
}

fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, error_location: &str, message: &str) {
    println!("[line {}] error {}: {}", line, error_location, message);
}
