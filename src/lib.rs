use std::fs;
use std::io::{stdin, Error, ErrorKind};
use input_stream::InputStream;
use scanner::{TokenType, Scanner, Token};

use crate::parser::Parser;

mod scanner;
mod ast;
mod parser;
mod interpreter;
mod environment;

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn run_file(path: &String) -> std::io::Result<()> {
        let contents = fs::read_to_string(path)?;
        Lox::run(contents);
        Ok(())
    }

    pub fn run_prompt() -> std::io::Result<()> {
        let stdin = stdin();
        let mut input = InputStream::new(stdin.lock());
        loop {
            print!("> ");
            let cmd: String = match input.scan() {
                Ok(cmd) => cmd,
                Err(_) => break,
            };
            Lox::run(cmd);
        }
        Ok(())
    }

    pub fn run(source: String) {
        let mut scanner = Scanner::new(source.as_str());
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        if parser.had_error {
            println!("Parser error.");
            return;
        }
        let mut interpreter = interpreter::Interpreter{
            environment: environment::Environment::new(),
        };
        match statements {
            Ok(stmt) => {
                for stmt in stmt {
                    interpreter.interpret(stmt);
                }
            },
            Err(err) => panic!("{}", err),
        }
    }

    pub fn error(token: &Token, message: String) -> String {
        if token.token_type == TokenType::EoF {
            format!("Error on line {} at end. {}", token.line, message)
        } else {
            format!("Error on line {} at '{}'. {}", token.line, token.lexeme, message)
        }
    }

    pub fn runtime_error(message: String) {
        println!("{}", message);
    }

}


