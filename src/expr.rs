use crate::token::{Token, TokenType};
use crate::vari::VariTypes;

pub enum Expr {
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Unary {
        op: Token,
        rhs: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        value: Box<VariTypes>,
    },
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: Expr) -> T;
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, expr: Expr) {
        let res = self.evaluate(expr);
        println!("{}", self.stringify(*res));
    }

    pub fn stringify(&self, vari_obj: VariTypes) -> String {
        match vari_obj {
            VariTypes::Nil => {
                return "nil".to_owned();
            }
            VariTypes::Num(v) => return v.to_string(),
            VariTypes::Boolean(b) => return b.to_string(),
            VariTypes::String(s) => return s,
            VariTypes::Object(o) => return "[object]".to_owned(),
        }
    }

    pub fn evaluate(&self, expr: Expr) -> Box<VariTypes> {
        return self.visit(expr);
    }

    fn check_type(&self, a: Box<VariTypes>, e: VariTypes) -> bool {
        match e {
            VariTypes::Nil => return matches!(*a, VariTypes::Nil),
            VariTypes::Num(_) => return matches!(*a, VariTypes::Num(_)),
            VariTypes::String(_) => return matches!(*a, VariTypes::String(_)),
            VariTypes::Boolean(_) => return matches!(*a, VariTypes::Boolean(_)),
            VariTypes::Object(_) => return matches!(*a, VariTypes::Object(_)),
        }
    }

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

    fn is_true(&self, object: Box<VariTypes>) -> bool {
        match *object {
            VariTypes::Nil => return false,
            VariTypes::Boolean(b) => return b,
            _ => return true,
        }
    }

    fn visit(&self, expr: Expr) -> Box<VariTypes> {
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
                        return Box::new(VariTypes::Boolean(self.is_true(right)));
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
                todo!()
            }
            Expr::Literal { value } => {
                todo!()
            }
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<Expr>) -> String {
        let mut expr_str: String = "( ".to_owned();
        expr_str += name.as_str();

        for expr in exprs {
            expr_str += " ";
            expr_str += self.visit_expr(expr).as_str();
        }

        expr_str += ")";

        expr_str
    }

    pub fn print(&mut self, expr: Expr) -> String {
        self.visit_expr(expr)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Binary { lhs, op, rhs } => {
                let exprs = vec![*lhs, *rhs];
                self.parenthesize(op.lexeme, exprs)
            }
            Expr::Unary { op, rhs } => {
                let exprs = vec![*rhs];
                self.parenthesize(op.lexeme, exprs)
            }
            Expr::Literal { value } => match *value {
                VariTypes::Nil => "nil".to_owned(),
                VariTypes::Num(fp) => fp.to_string(),
                VariTypes::Boolean(b) => b.to_string(),
                VariTypes::String(strval) => strval,
                VariTypes::Object(_any_value) => "todo".to_string(),
            },
            Expr::Grouping { expr } => self.parenthesize("group".to_owned(), vec![*expr]),
        }
    }
}
