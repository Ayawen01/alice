use crate::{token::Token, error::AliceError};

pub trait VisitExpr<R> {
    fn visit_grouping_expr(&mut self, expression: Expr) -> Result<R, AliceError>;
    fn visit_variable_expr(&mut self, name: Token) -> Result<R, AliceError>;
    fn visit_assign_expr(&mut self, name: Token, value: Expr) -> Result<R, AliceError>;
    fn visit_unary_expr(&mut self, operator: Token, value: Expr) -> Result<R, AliceError>;
    fn visit_binary_expr(&mut self, left: Expr, operator: Token, right: Expr) -> Result<R, AliceError>;
    fn visit_logical_expr(&mut self, left: Expr, operator: Token, right: Expr) -> Result<R, AliceError>;
    fn visit_call_expr(&mut self, callee: Expr, paren: Token, arguments: Vec<Expr>) -> Result<R, AliceError>;
    fn visit_literal_expr(&mut self, value: AliceObject) -> Result<R, AliceError>;
    fn visit_array_expr(&mut self, list: Vec<Expr>) -> Result<R, AliceError>;

    fn evaluate(&mut self, expr: Expr) -> Result<R, AliceError> {
        match expr {
            Expr::Grouping { expression } => self.visit_grouping_expr(*expression),
            Expr::Variable { name } => self.visit_variable_expr(name),
            Expr::Assign { name, value } => self.visit_assign_expr(name, *value),
            Expr::Unary { operator, value } => self.visit_unary_expr(operator, *value),
            Expr::Binary { left, operator, right } => self.visit_binary_expr(*left, operator, *right),
            Expr::Logical { left, operator, right } => self.visit_logical_expr(*left, operator, *right),
            Expr::Call { callee, paren, arguments } => self.visit_call_expr(*callee, paren, arguments),
            Expr::Literal { value } => self.visit_literal_expr(value),
            Expr::Array { value } => self.visit_array_expr(value)
        }
    }
}

#[derive(Debug, Clone)]
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
        value: AliceObject
    },
    Array {
        value: Vec<Expr>
    }
}

#[derive(Debug, Clone)]
pub enum AliceObject {
    Array(Vec<AliceObject>),
    String(String),
    F64(f64),
    I64(i64),
    Boolean(bool),
    Nil
}

pub trait VisitStmt<R> {
    fn visit_println_stmt(&mut self, expression: Expr) -> Result<R, AliceError>;
    fn visit_return_stmt(&mut self, keyword: Token, value: Option<Expr>) -> Result<R, AliceError>;
    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> Result<R, AliceError>;
    fn visit_block_stmt(&mut self, statements: Vec<Stmt>) -> Result<R, AliceError>;
    fn visit_fn_stmt(&mut self, name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Result<R, AliceError>;
    fn visit_if_stmt(&mut self, condition: Expr, then_branch: Stmt, else_branch: Option<Box<Stmt>>) -> Result<R, AliceError>;
    fn visit_for_stmt(&mut self, value: Token, expression: Expr, body: Vec<Stmt>) -> Result<R, AliceError>;
    fn visit_expression_stmt(&mut self, expression: Expr) -> Result<R, AliceError>;

    fn execute(&mut self, stmt: Stmt) -> Result<R, AliceError> {
        match stmt {
            Stmt::Println { expression } => self.visit_println_stmt(expression),
            Stmt::Return { keyword, value } => self.visit_return_stmt(keyword, value),
            Stmt::Var { name, initializer } => self.visit_var_stmt(name, initializer),
            Stmt::Block { statements } => self.visit_block_stmt(statements),
            Stmt::Fn { name, params, body } => self.visit_fn_stmt(name, params, body),
            Stmt::If { condition, then_branch, else_branch } => self.visit_if_stmt(condition, *then_branch, else_branch),
            Stmt::For { value, expression, body } => self.visit_for_stmt(value, expression, body),
            Stmt::Expression { expression } => self.visit_expression_stmt(expression)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Println {
        expression: Expr
    },
    Return {
        keyword: Token,
        value: Option<Expr>
    },
    Var {
        name: Token,
        initializer: Option<Expr>
    },
    Block {
        statements: Vec<Stmt>
    },
    Fn {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    For {
        value: Token,
        expression: Expr,
        body: Vec<Stmt>
    },
    Expression {
        expression: Expr
    }
}