use std::env::args;
use lox::Lox;

fn main() {

    let args: Vec<String> = args().collect();

    match args.len() {
        length if length > 2 => {
            println!("Usage: loxrs [script]");
        } 
        length if length == 2  => Lox::run_file(&args[1]).unwrap(),
        _ => Lox::run_prompt().unwrap(),
    }

}

