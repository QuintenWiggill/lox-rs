use crate::Lox;
use crate::scanner::{Token, TokenType};
use crate::ast::{ Expr, Value };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    } 
    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality() 
    }
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
            
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(
            vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]
        ) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(
            vec![TokenType::Minus, TokenType::Plus]
        ) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(
            vec![TokenType::Slash, TokenType::Star]
        ) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }
        Ok(expr)
    } 

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right)
            })
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal { value: Value::Boolean(false) })
        }
        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal { value: Value::Boolean(true) })
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::Literal { value: Value::Nil })
        }
        if self.match_token(vec![TokenType::NumberLiteral]) {
            return Ok(Expr::Literal { 
                value: Value::Number(self.previous().lexeme.parse::<f64>().unwrap())
            })
        }
        if self.match_token(vec![TokenType::StringLiteral]) {
            return Ok(Expr::Literal {
                value: Value::String(self.previous().lexeme.clone())
            })
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;  
            self.consume(TokenType::RightParen, String::from("Expect ')' after expression."));
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        };
        Err(self.parse_error(self.peek(), "Expect expression.".to_string()))
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
       for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        } 
        false
    }

    fn consume(&mut self, t: TokenType, message: String) {
        if self.check(t) { 
            self.advance();
        }
        self.parse_error(self.peek(), message); 
    }

    fn parse_error(&self, token: &Token, message: String) -> String {
        Lox::error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance(); 
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => self.advance()       
            };
        };
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == t
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EoF
    } 
   
    // if we're at the end of the tokens, return the last token
    fn peek(&self) -> &Token {
        match self.tokens.get(self.current) {
            Some(token) => token,
            None => self.tokens.last().unwrap(),
        }
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
