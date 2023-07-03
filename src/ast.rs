use crate::scanner::Token;

pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl AstPrinter for Value {
    fn print(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_owned(),
            Value::Boolean(b) => b.to_string(),
            Value::Nil => String::from("nil")
        }
    }
}

impl AstPrinter for Expr {
    fn print(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => self.parenthesize(&operator.lexeme, vec![left, right]),
            Expr::Grouping { expression } => self.parenthesize(&"group".to_string(), vec![expression]),
            Expr::Literal { value } => value.print(),
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, vec![right]),
            _ => String::new()
        }
    }
}

pub trait AstPrinter {
    fn print(&self) -> String;

    fn parenthesize(&self, name: &String, exprs: Vec<&Expr>) -> String {
        let mut builder = format!("({}", name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(expr.print().as_str());
        }
        builder.push(')');
        builder
    }
}

