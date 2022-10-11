use crate::{expr::Expr, token::Token};

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
    // name, parameters, body
    Function(Token, Vec<Token>, Vec<Stmt>),
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: Stmt) -> T;
}
