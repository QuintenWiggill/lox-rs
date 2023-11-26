use crate::scanner::Token;

#[derive(Clone)]
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
#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        superclass: Option<Expr>,
        methods: Vec<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
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

impl AstPrinter for Stmt {
    fn print(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.print(),
            Stmt::Print { expression } => expression.print(),
            Stmt::Var { name, initializer } => {
                if let Some(expr) = initializer {
                    self.parenthesize(&"var".to_string(), vec![&Expr::Variable { name: name.clone() }, expr])
                } else {
                    self.parenthesize(&"var".to_string(), vec![&Expr::Variable { name: name.clone() }])
                }
            }
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

