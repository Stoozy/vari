use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    environment::Environment, interpreter::Interpreter, stmt::Stmt, token::Token, vari::VariError,
    vari::VariTypes,
};

#[derive(Clone)]
pub enum Procedure {
    Native {
        name: String,
        arity: usize,
        body: Box<fn(&Vec<VariTypes>) -> VariTypes>,
    },
    User {
        arity: usize,
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl fmt::Debug for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native { arity, .. } => f
                .debug_struct("[native function]")
                .field("parameters", arity)
                .finish(),
            Self::User { arity, name, .. } => f
                .debug_struct("[function]")
                .field("parameters", arity)
                .field("name", &name.lexeme)
                .finish(),
        }
    }
}

impl Procedure {
    pub fn arity(&self) -> usize {
        match &*self {
            Procedure::Native { arity, .. } => {
                return *arity;
            }
            Procedure::User { arity, .. } => {
                return *arity;
            }
        }
    }

    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<VariTypes>) -> VariTypes {
        match &*self {
            Procedure::Native { body, .. } => (*body)(&args),
            Procedure::User {
                params,
                body,
                closure,
                ..
            } => {
                let env = Rc::new(RefCell::new(Environment::from(closure)));

                for (param, arg) in params.iter().zip(args.iter()) {
                    env.borrow_mut().define(param.lexeme.clone(), arg.clone());
                }

                match interpreter.execute_block((*body).clone(), env) {
                    Err(VariError::Return(retval)) => {
                        return retval;
                    }
                    // No return value,
                    // so return nil by default
                    Ok(_) => return VariTypes::Nil,
                }
            }
        }
    }
}
