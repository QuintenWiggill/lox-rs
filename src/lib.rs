use std::fs;
use std::io::{stdin, Error, ErrorKind};
use input_stream::InputStream;
use scanner::{TokenType, Scanner, Token};
use ast::{Expr, Value, AstPrinter};

mod scanner;
mod ast;
mod parser;

pub struct Lox {}

impl Lox {
    pub fn run_file(path: &String) -> std::io::Result<()> {
        let contents = fs::read_to_string(path)?;
        Lox::run(contents).unwrap();
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
            Lox::run(cmd).map_err(|err| Error::new(ErrorKind::Other, err))?;
        }
        Ok(())
    }

    pub fn run(source: String) -> Result<(), String> {
        let mut scanner = Scanner::new(source.as_str());
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        let expression =  Expr::Binary { 
            left: Box::new(Expr::Unary { 
                operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), line: 1 },
                right: Box::new(Expr::Literal { value: Value::Number(123.0) })
                }),
            operator: Token { token_type: TokenType::Star, lexeme: "*".to_string(), line: 1 },
            right: Box::new(
                Expr::Grouping { expression: Box::new(Expr::Literal { value: Value::Number(45.67) })}
            )
        };
        println!("{}", expression.print());
        Ok(())
    }

}


