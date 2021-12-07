use std::io;
use std::env;
use std::process;

struct Lox {}

impl Lox {
    fn run_file(&mut self, path: &str) {
        let content = std::fs::read_to_string(path).expect("File not found");
        run(&content);
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(n) => {
                    match n {
                        0 => { break; }
                        _ => { run(line) }
                    }
                }
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            }
        }
    }

    fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens.iter() {
            println!(token)
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => { run_prompt(); }
        2 => { run_file(&args[1]); }
        _ => {
            println!("Usage: rustlox [script]");
            process::exit(64);
        }
    }
}