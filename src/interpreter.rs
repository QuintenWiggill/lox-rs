use crate::{ast::{ Expr, Value, AstPrinter, Stmt }, scanner::TokenType, Lox, environment::Environment};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression { expression } => match self.evaluate(expression) {
                Ok(_) => (),
                Err(msg) => Lox::runtime_error(msg)
            }
            Stmt::Print { expression } => match self.evaluate(expression) {
                Ok(val) => println!("{}", val.print()),
                Err(msg) => Lox::runtime_error(msg)
            } 
            Stmt::Var { name, initializer } => {
                let val = match initializer {
                    Some(expr) => self.evaluate(expr),
                    None => Ok(Value::Nil)
                };
                self.environment.define(name.lexeme, val.unwrap());
            }
            _ => Lox::runtime_error(String::from("Not implemented."))
        } 
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Assign { name, value } => {
                let val = self.evaluate(*value);
                self.environment.assign(name, val.unwrap())
            }
            Expr::Variable { name } => self.environment.get(&name),
            Expr::Literal { value } => Ok(value),
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(*right)?;

                match operator.token_type {
                    TokenType::Bang => Ok(Value::Boolean(!self.is_truthy(right))),
                    TokenType::Minus => match right {
                        Value::Number(num) => Ok(Value::Number(-(num))),
                        _ => Err("Not a valid operand".to_string())
                    }
                    _ => Err("Unknown unary operator.".to_string())
                }
            }
            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                match operator.token_type {
                    TokenType::Greater => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Boolean(lnum > rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::GreaterEqual => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Boolean(lnum >= rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::Less => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Boolean(lnum < rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::LessEqual => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Boolean(lnum <= rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::Minus => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Number(lnum - rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::Slash => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Number(lnum / rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::Star => {
                        match (left, right) {
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Number(lnum * rnum)),
                            (_, _) => Err("Operands must be numbers.".to_string())
                        }
                    }
                    TokenType::Plus => {
                       match (left, right) {
                            (Value::String(lstr), Value::String(rstr)) => Ok(Value::String(format!("{lstr}{rstr}"))),
                            (Value::Number(lnum), Value::Number(rnum)) => Ok(Value::Number(lnum + rnum)),
                            (_, _) => Err("Invalid operator for operands".to_string())
                        } 
                    }
                    TokenType::BangEqual => Ok(Value::Boolean(!self.is_equal(left, right))),
                    TokenType::EqualEqual => Ok(Value::Boolean(self.is_equal(left, right))),
                    _ => Err("Unkown binary operator".to_string()),
                }
            }
            _ => Err("Not an expression".to_string())
        }
    }

    fn is_equal(&self, left: Value, right: Value) -> bool {
        match (left, right) {
            (Value::String(lstr), Value::String(rstr)) => lstr == rstr,
            (Value::Number(lnum), Value::Number(rnum)) => lnum == rnum,
            (Value::Boolean(lbool), Value::Boolean(rbool)) => lbool == rbool,
            (Value::Nil, Value::Nil) => true,
            (_, _) => false,
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Boolean(boolean) => boolean,
            _ => true,
        }
    }
} 
