mod error;
mod expr;
mod lexer;
mod lox;

use crate::lox::Lox;
use std::{env, process};

fn main() {
    let mut lox = Lox::default();
    let args = env::args();
    if args.len() == 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() > 1 {
        let argvs: Vec<String> = args.collect();
        _ = lox.run_file(argvs.get(1).unwrap().to_string());
    } else {
        _ = lox.run_prompt();
    }
}
