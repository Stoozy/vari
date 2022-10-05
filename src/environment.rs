use crate::{token::Token, vari::VariTypes};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, VariTypes>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: Token) -> VariTypes {
        if let Some(value) = self.values.get(&name.lexeme) {
            return (*value).clone();
        }

        todo!()
        // syntax error "undefined vairable"
    }

    pub fn assign(&mut self, name: String, value: VariTypes) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }

        todo!()
        // error: undefined variable
    }

    pub fn define(&mut self, name: String, value: VariTypes) {
        self.values.insert(name, value);
    }
}
