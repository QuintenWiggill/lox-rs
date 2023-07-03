use crate::scanner::{Token, TokenType};
use crate::ast::{ Expr, Value };

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn expression(&mut self) -> Expr {
        self.equality() 
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
            
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(
            vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]
        ) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(
            vec![TokenType::Minus, TokenType::Plus]
        ) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(
            vec![TokenType::Slash, TokenType::Star]
        ) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }
        expr
    } 

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right)
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::False]) {
            return Expr::Literal { value: Value::Boolean(false) }
        }
        if self.match_token(vec![TokenType::True]) {
            return Expr::Literal { value: Value::Boolean(true) }
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Expr::Literal { value: Value::Nil }
        }

        if self.match_token(vec![TokenType::NumberLiteral]) {
            return Expr::Literal { 
                value: Value::Number(self.previous().lexeme.parse::<f64>().unwrap())
            }
        }

        if self.match_token(vec![TokenType::StringLiteral]) {
            return Expr::Literal {
                value: Value::String(self.previous().lexeme.clone())
            }
        }

        // if self.match_token(vec![TokenType::LeftParen]) {
        else {
            let expr = self.expression();  
            return Expr::Grouping { expression: Box::new(expr) };
        }

        
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
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }
    
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
