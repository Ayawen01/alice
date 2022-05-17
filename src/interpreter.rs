use std::{rc::Rc, cell::RefCell};

use crate::{environment::Environment, ast::{Expr, Stmt, AliceObject, VisitExpr, VisitStmt}, error::AliceError, token::{Token, TokenType, Literal}};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>
}

impl Interpreter {
    #[inline]
    pub fn new() -> Interpreter {
        Interpreter { environment: Rc::new(RefCell::new(Environment::new())) }
    }

    #[inline]
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), AliceError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        
        Ok(())
    }
    
    #[inline]
    fn is_truthy(&self, value: &AliceObject) -> bool {
        match value {
            AliceObject::Nil => false,
            AliceObject::Boolean(bool) => *bool,
            _ => true        
        }
    }

    #[inline]
    fn is_equal(&self, l: &AliceObject, r: &AliceObject) -> bool {
        match (l, r) {
            (AliceObject::Nil, AliceObject::Nil) => true,
            (AliceObject::Boolean(l), AliceObject::Boolean(r)) => l == r,
            (AliceObject::String(l), AliceObject::String(r)) => l == r,
            (AliceObject::F64(l), AliceObject::F64(r)) => l == r,
            (AliceObject::I64(l), AliceObject::I64(r)) => l == r,
            _ => false
        }
    }

    #[inline]
    fn stringify(&self, value: AliceObject) -> String {
        match value {
            AliceObject::String(str) => str,
            AliceObject::Array(list) => format!("{:?}", list),
            AliceObject::F64(num) => num.to_string(),
            AliceObject::I64(num) => num.to_string(),
            AliceObject::Boolean(bool) => bool.to_string(),
            AliceObject::Nil => "nil".to_owned()
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), AliceError> {
        let previous = environment.clone();

        self.environment = Rc::new(RefCell::new(Environment::from(environment)));

        for stmt in statements {
            self.execute(stmt)?;
        }

        self.environment = previous;

        Ok(())
    }
}

impl VisitExpr<AliceObject> for Interpreter {
    fn visit_grouping_expr(&mut self, expression: Expr) -> Result<AliceObject, AliceError> {
        self.evaluate(expression)
    }

    fn visit_variable_expr(&mut self, name: Token) -> Result<AliceObject, AliceError> {
        self.environment.borrow().get(name)
    }

    fn visit_assign_expr(&mut self, name: Token, value: Expr) -> Result<AliceObject, AliceError> {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value)
    }

    fn visit_unary_expr(&mut self, operator: Token, value: Expr) -> Result<AliceObject, AliceError> {
        let value = self.evaluate(value)?;
        
        Ok(match operator.r#type {
            TokenType::Bang => AliceObject::Boolean(!self.is_truthy(&value)),
            TokenType::Minus => {
                if let AliceObject::F64(num) = value {
                    AliceObject::F64(-num)
                } else if let AliceObject::I64(num) = value {
                    AliceObject::I64(-num)
                } else {
                    let msg = format!("{:?} must be a number.", value);
                    return Err(AliceError::RuntimeError(msg.into(), operator.line));
                }
            }
            _ => AliceObject::Nil
        })
    }

    fn visit_binary_expr(&mut self, left: Expr, operator: Token, right: Expr) -> Result<AliceObject, AliceError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        
        match operator.r#type {
            TokenType::BangEqual => Ok(AliceObject::Boolean(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(AliceObject::Boolean(self.is_equal(&left, &right))),
            TokenType::Greater => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l > r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l > r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            },
            TokenType::GreaterEqual => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l >= r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l >= r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            },
            TokenType::Less => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l < r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l < r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            },
            TokenType::LessEqual => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l <= r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::Boolean(l <= r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            },
            TokenType::Minus => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::F64(l - r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::I64(l - r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            }
            TokenType::Plus => {
                match (left.clone(), right.clone()) {
                    (AliceObject::String(l), AliceObject::String(r)) => Ok(AliceObject::String(l + &r)),
                    (AliceObject::F64(l), AliceObject::F64(r)) => Ok(AliceObject::F64(l + r)),
                    (AliceObject::I64(l), AliceObject::I64(r)) => Ok(AliceObject::I64(l + r)),
                    _ => {
                        let msg = format!("{:?} and {:?} must both be numbers or both be strings.", left, right);
                        Err(AliceError::RuntimeError(msg.into(), operator.line))
                    }
                }
            }
            TokenType::Slash => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::F64(l / r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::I64(l / r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            }
            TokenType::Star => {
                if let (AliceObject::F64(l), AliceObject::F64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::F64(l * r))
                } else if let (AliceObject::I64(l), AliceObject::I64(r)) = (left.clone(), right.clone()) {
                    Ok(AliceObject::I64(l * r))
                } else {
                    let msg = format!("{:?} and {:?} must be numbers.", left, right);
                    Err(AliceError::RuntimeError(msg.into(), operator.line))
                }
            }
            _ => {
                Ok(AliceObject::Nil)
            }
        }
    }

    fn visit_logical_expr(&mut self, left: Expr, operator: Token, right: Expr) -> Result<AliceObject, AliceError> {
        let left = self.evaluate(left)?;

        if operator.r#type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(right)
    }

    fn visit_call_expr(&mut self, callee: Expr, paren: Token, arguments: Vec<Expr>) -> Result<AliceObject, AliceError> {
        todo!()
    }

    fn visit_literal_expr(&mut self, value: AliceObject) -> Result<AliceObject, AliceError> {
        Ok(value)
    }

    fn visit_array_expr(&mut self, list: Vec<Expr>) -> Result<AliceObject, AliceError> {
        let mut values = Vec::new();
        for expr in list {
            values.push(self.evaluate(expr)?);
        }
        Ok(AliceObject::Array(values))
    }
}

impl VisitStmt<()> for Interpreter {
    fn visit_println_stmt(&mut self, expression: Expr) -> Result<(), AliceError> {
        match self.evaluate(expression) {
            Ok(expr) => {
                println!("{}", self.stringify(expr));
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn visit_return_stmt(&mut self, keyword: Token, value: Option<Expr>) -> Result<(), AliceError> {
        todo!()
    }

    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> Result<(), AliceError> {
        if let Some(expr) = initializer {
            match self.evaluate(expr) {
                Ok(v) => {
                    self.environment.borrow_mut().define(name.lexeme.unwrap(), v);
                }
                Err(e) => return Err(e)
            };
        } else {
            self.environment.borrow_mut().define(name.lexeme.unwrap(), AliceObject::Nil);
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: Vec<Stmt>) -> Result<(), AliceError> {
        self.execute_block(statements, self.environment.clone())
    }

    fn visit_fn_stmt(&mut self, name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Result<(), AliceError> {
        todo!()
    }

    fn visit_if_stmt(&mut self, condition: Expr, then_branch: Stmt, else_branch: Option<Box<Stmt>>) -> Result<(), AliceError> {
        let v = self.evaluate(condition)?;
        if self.is_truthy(&v) {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(*else_branch)?;
        }
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: Expr) -> Result<(), AliceError> {
        match self.evaluate(expression) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}