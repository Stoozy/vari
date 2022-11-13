use std::collections::HashMap;

use crate::{environment::Environment, expr::Expr, token::Token, vari::VariTypes};

#[derive(Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    // name, initializer
    Var(Token, Option<Expr>),
    // condition, if branch, else branch
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    // condition, body
    While(Expr, Box<Stmt>),
    // name, parameters, body, closure
    Function(Token, Vec<Token>, Vec<Stmt>),
    Return(Token, Expr),
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: Stmt) -> T;
}
