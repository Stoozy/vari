use crate::environment::Environment;
use crate::expr::{Expr, ExprVisitor};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::TokenType;
use crate::vari::VariTypes;

#[derive(Debug, Clone)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    fn execute(&mut self, statement: Stmt) {
        self.visit_stmt(statement);
    }

    fn execute_block(&mut self, stmts: Vec<Stmt>, env: Environment) {
        let tmp_env = self.env.clone();

        // new env for new block scope
        self.env = env;

        for stmt in stmts {
            self.execute(stmt);
        }

        // then switch back to the original scopes environment
        self.env = tmp_env;
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    pub fn stringify(&self, vari_obj: VariTypes) -> String {
        // TODO: maybe move this to the enums to_string() ?
        match vari_obj {
            VariTypes::Nil => {
                return "nil".to_owned();
            }
            VariTypes::Num(v) => return v.to_string(),
            VariTypes::Boolean(b) => return b.to_string(),
            VariTypes::String(s) => return s,
            VariTypes::Object(_) => return "[object]".to_owned(),
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Box<VariTypes> {
        return self.visit_expr(expr);
    }

    // for future error handling
    fn _check_type(&self, a: Box<VariTypes>, e: VariTypes) -> bool {
        match e {
            VariTypes::Nil => return matches!(*a, VariTypes::Nil),
            VariTypes::Num(_) => return matches!(*a, VariTypes::Num(_)),
            VariTypes::String(_) => return matches!(*a, VariTypes::String(_)),
            VariTypes::Boolean(_) => return matches!(*a, VariTypes::Boolean(_)),
            VariTypes::Object(_) => return matches!(*a, VariTypes::Object(_)),
        }
    }

    // problematic (true == false)
    fn is_equal(&self, a: Box<VariTypes>, b: Box<VariTypes>) -> bool {
        if let (VariTypes::Nil, VariTypes::Nil) = (*a.clone(), *b.clone()) {
            return true;
        }

        if let VariTypes::Nil = *a.clone() {
            return false;
        }

        if let VariTypes::Nil = *b.clone() {
            return false;
        }

        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*a.clone(), *b.clone()) {
            return l.to_bits() == r.to_bits();
        }

        if let (VariTypes::String(l), VariTypes::String(r)) = (*a.clone(), *b.clone()) {
            return l == r;
        }

        if let (VariTypes::Boolean(l), VariTypes::Boolean(r)) = (*a.clone(), *b.clone()) {
            return l == r;
        }

        // TODO: handle VariTypes::Objects

        return false;
    }

    fn is_true(&mut self, object: Box<VariTypes>) -> bool {
        match *object {
            VariTypes::Nil => return false,
            VariTypes::Boolean(b) => return b,
            _ => return true,
        }
    }
}

impl ExprVisitor<Box<VariTypes>> for Interpreter {
    fn visit_expr(&mut self, expr: Expr) -> Box<VariTypes> {
        match expr {
            Expr::Binary { lhs, op, rhs } => {
                let left = self.evaluate(*lhs);
                let right = self.evaluate(*rhs);

                match op.token_type {
                    TokenType::MINUS => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Num(l - r));
                        }
                    }
                    TokenType::SLASH => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Num(l / r));
                        }
                    }
                    TokenType::STAR => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Num(l * r));
                        }
                    }
                    TokenType::PLUS => {
                        // handling addition of numbers
                        if let (VariTypes::Num(l), VariTypes::Num(r)) =
                            (*left.clone(), *right.clone())
                        {
                            return Box::new(VariTypes::Num(l + r));
                        }

                        // handling addition of string type
                        if let (VariTypes::String(s1), VariTypes::String(s2)) = (*left, *right) {
                            return Box::new(VariTypes::String(format!("{}{}", s1, s2)));
                        }
                    }

                    TokenType::GT => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Boolean(l > r));
                        }
                    }
                    TokenType::LT => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Boolean(l < r));
                        }
                    }
                    TokenType::GE => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Boolean(l >= r));
                        }
                    }
                    TokenType::LE => {
                        if let (VariTypes::Num(l), VariTypes::Num(r)) = (*left, *right) {
                            return Box::new(VariTypes::Boolean(l <= r));
                        }
                    }
                    TokenType::NE => {
                        return Box::new(VariTypes::Boolean(!self.is_equal(left, right)))
                    }
                    TokenType::ISEQ => {
                        return Box::new(VariTypes::Boolean(self.is_equal(left, right)))
                    }
                    _ => unreachable!(),
                }

                unreachable!()
            }
            Expr::Unary { op, rhs } => {
                let right = self.evaluate(*rhs);
                match op.token_type {
                    TokenType::NOT => {
                        return Box::new(VariTypes::Boolean(!self.is_true(right)));
                    }
                    TokenType::MINUS => {
                        if let VariTypes::Num(num) = *right {
                            return Box::new(VariTypes::Num(-num));
                        }

                        unreachable!();
                    }
                    _ => {}
                }

                unreachable!();
            }
            Expr::Grouping { expr } => {
                return self.evaluate(*expr);
            }
            Expr::Literal { value } => {
                return value;
            }
            Expr::Variable { value } => {
                return Box::new(self.env.get(value));
            }
            Expr::Assign { name, value_expr } => {
                let value = self.evaluate(*value_expr);
                self.env.assign(name.lexeme, (*value).clone());
                return value;
            }
            Expr::Logical { lhs, operator, rhs } => {
                let lhs = self.evaluate(*lhs);

                if operator.token_type == TokenType::OR {
                    if self.is_true(lhs.clone()) {
                        return lhs;
                    } else {
                        if !self.is_true(lhs.clone()) {
                            return lhs;
                        }
                    }
                }

                return self.evaluate(*rhs);
            }
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
            Stmt::Print(expr) => {
                let val = self.evaluate(expr);
                println!("{}", self.stringify(*val));
            }
            Stmt::Var(name, initializer) => match initializer {
                Some(expr_val) => {
                    let val = self.evaluate(expr_val);
                    self.env.define(name.lexeme, *val);
                }
                None => {
                    self.env.define(name.lexeme, VariTypes::Nil);
                }
            },
            Stmt::Block(stmt_list) => {
                self.execute_block(
                    stmt_list,
                    Environment::new().set_enclosing(Box::new(self.env.clone())),
                );
            }
            Stmt::If(conditional_expr, then_block, else_block) => {
                let res = self.evaluate(conditional_expr);
                if self.is_true(res) {
                    self.execute(*then_block);
                } else if let Some(else_stmt) = else_block {
                    self.execute(*else_stmt);
                }
            }
        }
    }
}
