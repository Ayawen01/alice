use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{ast::AliceObject, error::AliceError, token::Token};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, AliceObject>,
    environment: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new(), environment: None }
    }

    pub fn from(environment: Rc<RefCell<Environment>>) -> Environment {
        Environment { values: HashMap::new(), environment: Some(environment) }
    }

    pub fn get(&self, name: Token) -> Result<AliceObject, AliceError> {
        let lexeme = name.clone().lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.get(&lexeme).unwrap().clone())
        } else {
            if let Some(environment) = &self.environment {
                environment.borrow().get(name)
            } else {
                Err(AliceError::RuntimeError(format!("Undefined variable '{}'.", &lexeme).into(), name.line))
            }
        }
    }

    pub fn define(&mut self, name: String, value: AliceObject) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: AliceObject) -> Result<AliceObject, AliceError> {
        let lexeme = name.clone().lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.insert(lexeme, value).unwrap())
        } else {
            if let Some(environment) = &mut self.environment {
                environment.borrow_mut().assign(name, value)
            } else {
                Err(AliceError::RuntimeError(format!("Undefined variable '{}'.", lexeme).into(), name.line))   
            }
        }
    }   
}
