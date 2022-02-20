fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: biloxi [script]");
    } else if args.len() == 1 {
        run_file(std::path::Path::new(&args[0]));
    } else {
        run_prompt();
    }
}

fn run_file(file: &std::path::Path) {
    println!("{}", file.display());
}

fn run_prompt() {
    println!("repl!");
}
