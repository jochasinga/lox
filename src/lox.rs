use crate::lexer::{Scanner, Token};
use std::{fs, io, process};

#[derive(Debug, Default)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn run_file(&self, path: String) -> Result<(), io::Error> {
        match fs::read_to_string(path) {
            Ok(content) => {
                Self::run(content);
                if self.had_error {
                    process::exit(65);
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let mut buffer = String::new();
        loop {
            println!("> ");
            io::stdin().read_line(&mut buffer)?;
            Self::run(buffer.clone());
            self.had_error = false;
        }
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        // For now, just print the tokens.
        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, hint: String, message: String) {
        println!("[line {line}] Error {hint}: {message}");
        self.had_error = true;
    }
}
