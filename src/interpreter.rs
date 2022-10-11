use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::expr::{Expr, ExprVisitor};
use crate::procedure::Procedure;
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::TokenType;
use crate::vari::VariTypes;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    env: Rc<RefCell<Environment>>,
}

pub fn clock(args: &Vec<VariTypes>) -> VariTypes {
    VariTypes::Num(0.0)
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = RefCell::new(Environment::new());

        let clock_procedure = Procedure::Native {
            arity: 0,
            body: Box::new(clock),
        };

        globals
            .borrow_mut()
            .define("clock".to_owned(), VariTypes::Callable(clock_procedure));

        Self {
            globals: Rc::new(globals),
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    fn execute(&mut self, statement: Stmt) {
        self.visit_stmt(statement);
    }

    pub fn execute_block(&mut self, stmts: Vec<Stmt>, env: Rc<RefCell<Environment>>) {
        let tmp_env = self.env.clone();
        //let tmp_env = Box::new(self.env.clone());

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
            VariTypes::Callable(_) => return "[function]".to_owned(),
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
            VariTypes::Callable(_) => return matches!(*a, VariTypes::Object(_)),
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
                return Box::new(self.env.borrow_mut().get(value));
            }
            Expr::Assign { name, value_expr } => {
                let value = self.evaluate(*value_expr);
                self.env.borrow_mut().assign(name.lexeme, (*value).clone());
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
            Expr::Call {
                callee,
                paren,
                args,
            } => {
                // should just get the identifier of function name
                let callee = self.evaluate(*callee);

                let mut eval_args = vec![];

                for arg_expr in args {
                    eval_args.push(*self.evaluate(arg_expr));
                }

                match *callee {
                    VariTypes::Callable(pro) => {
                        if eval_args.len() == pro.arity() {
                            return Box::new(pro.call(self, eval_args));
                        } else {
                            // TODO: error handling
                            panic!(
                                "Expected {} arguments but got {}.",
                                pro.arity(),
                                eval_args.len()
                            );
                        }
                    }
                    _ => {
                        panic!("Can only call functions.");
                    }
                }
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
                    self.env.borrow_mut().define(name.lexeme, *val);
                }
                None => {
                    self.env.borrow_mut().define(name.lexeme, VariTypes::Nil);
                }
            },
            Stmt::Block(stmt_list) => {
                // send the *actual* env to `from` method in Env
                let env_clone = Rc::new(RefCell::new(Environment::from(&self.env)));
                self.execute_block(stmt_list, env_clone);
            }
            Stmt::If(conditional_expr, then_block, else_block) => {
                let val = self.evaluate(conditional_expr);
                if self.is_true(val) {
                    self.execute(*then_block);
                } else if let Some(else_stmt) = else_block {
                    self.execute(*else_stmt);
                }
            }
            Stmt::While(conditional_expr, body) => {
                let mut val = self.evaluate(conditional_expr.clone());

                while self.is_true(val.clone()) {
                    self.execute((*body).clone());
                    val = self.evaluate(conditional_expr.clone());
                }
            }
            Stmt::Function(name, params, body) => {
                let procedure = Procedure::from(Stmt::Function(name.clone(), params, body));
                self.env
                    .borrow_mut()
                    .define(name.lexeme, VariTypes::Callable(procedure));
            }
        }
    }
}
