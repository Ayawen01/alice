use crate::token::Token;

pub enum Expr {
    Grouping {
        expression: Box<Expr>
    },
    Variable {
        name: Token
    },
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Unary {
        operator: Token,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>  
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    },
    Literal {
        value: AliceLiteral
    }
}

enum AliceLiteral {
    Array(Vec<AliceLiteral>),
    String(String),
    F64(f64),
    I64(i64),
    Boolean(bool),
    Nil
}

pub enum Stmt {
    
}