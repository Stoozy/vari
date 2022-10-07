use crate::{token::Token, vari::VariTypes};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, VariTypes>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn set_enclosing(mut self, environment: Box<Environment>) -> Environment {
        self.enclosing = Some(environment);
        self
    }

    pub fn get(&mut self, name: Token) -> VariTypes {
        if let Some(value) = self.values.get(&name.lexeme) {
            return (*value).clone();
        }

        if let Some(mut enclosing_env) = self.enclosing.clone() {
            return (*enclosing_env).get(name);
        }

        VariTypes::Nil
        //todo!()
        // syntax error "undefined vairable"
    }

    pub fn assign(&mut self, name: String, value: VariTypes) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }

        if let Some(mut enclosing_env) = self.enclosing.clone() {
            return (*enclosing_env).assign(name, value);
        }

        todo!()
        // error: undefined variable
    }

    pub fn define(&mut self, name: String, value: VariTypes) {
        self.values.insert(name, value);
    }
}
