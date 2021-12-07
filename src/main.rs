use std::io::{stdout, BufWriter};
use std::env;
use std::process;

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