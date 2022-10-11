use crate::{token::Token, vari::VariTypes};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, VariTypes>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn get(&mut self, token: Token) -> VariTypes {
        if let Some(value) = self.values.get(&token.lexeme) {
            return (*value).clone();
        }

        if let Some(enclosing_env) = self.enclosing.clone() {
            return (*enclosing_env).borrow_mut().get(token);
        }

        // TODO: syntax error "undefined vairable"
        panic!("Undefined variable {}", token.lexeme);
    }

    pub fn assign(&mut self, name: String, value: VariTypes) {
        if self.values.contains_key(&name) {
            self.values.insert(name.clone(), value.clone());
            return;
        }

        if let Some(enclosing_env) = self.enclosing.clone() {
            enclosing_env.borrow_mut().assign(name, value);
            return;
        }

        unreachable!()
        // error: undefined variable
    }

    pub fn define(&mut self, name: String, value: VariTypes) {
        println!("Defining {:?} as {}", value, name);
        self.values.insert(name, value);
    }
}

impl From<&Rc<RefCell<Environment>>> for Environment {
    fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }
    }
}
