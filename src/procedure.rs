use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    environment::Environment, interpreter::Interpreter, stmt::Stmt, token::Token, vari::VariTypes,
};

#[derive(Clone)]
pub enum Procedure {
    Native {
        arity: usize,
        body: Box<fn(&Vec<VariTypes>) -> VariTypes>,
    },
    User {
        arity: usize,
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
}

impl fmt::Debug for Procedure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native { arity, body } => f
                .debug_struct("[native function]")
                .field("parameters", arity)
                .finish(),
            Self::User {
                arity,
                name,
                params,
                body,
            } => f
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
            Procedure::Native { arity, body } => {
                return *arity;
            }
            Procedure::User {
                arity,
                name,
                params,
                body,
            } => {
                return *arity;
            }
        }
    }

    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<VariTypes>) -> VariTypes {
        match &*self {
            Procedure::Native { arity, body } => (*body)(&args),
            Procedure::User {
                arity,
                name,
                params,
                body,
            } => {
                let mut proc_env = Environment::from(&interpreter.globals);

                for it in params.iter().zip(args.iter()) {
                    let (param, arg) = it;
                    proc_env.define(param.lexeme.clone(), arg.clone());
                }

                interpreter.execute_block(body.clone(), Rc::new(RefCell::new(proc_env)));
                return VariTypes::Nil;
            }
        }
    }
}
impl From<Stmt> for Procedure {
    fn from(stmt: Stmt) -> Self {
        match stmt {
            Stmt::Function(name, params, body) => Procedure::User {
                arity: params.len(),
                name,
                params,
                body,
            },
            _ => {
                panic!("Must be Function type");
            }
        }
    }
}
