use crate::token::Token;

pub enum VariTypes {
    Void,
    Num(f64),
    String(String),
}

pub enum Expr {
    Binary {lhs: Box<Expr>, op: Token, rhs: Box<Expr>},
    Unary {op: Token, rhs:  Box<Expr>},
    Grouping{expr: Box::<Expr>},
    Literal{value: Box<VariTypes>}
}


pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: Expr) -> T;
}


pub struct AstPrinter;

impl AstPrinter {

    pub fn new() -> Self {
        Self
    }

    fn parenthesize (&mut self, name : String, exprs: Vec<Expr>) -> String {
        let mut expr_str : String = "( ".to_owned();
        expr_str += name.as_str(); 

        for expr in exprs {
            expr_str += " ";
            expr_str += self.visit_expr(expr).as_str();
        }

        expr_str += ")";

        expr_str
    }

    pub fn print(&mut self, expr : Expr ) -> String {
        self.visit_expr(expr)
    }
}


impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Binary { lhs, op, rhs } => {
                let exprs = vec![*lhs, *rhs];
                self.parenthesize(op.lexeme, exprs)
            },
            Expr::Unary { op, rhs } =>  {
                let exprs = vec![*rhs];
                self.parenthesize(op.lexeme, exprs)

            },
            Expr::Literal { value  } => {
                match *value {
                    VariTypes::Void => { "nil".to_owned()},
                    VariTypes::Num(fp) => { fp.to_string()},
                    VariTypes::String(strval) => { strval }
                }
            },
            Expr::Grouping { expr } => {
                self.parenthesize("group".to_owned(), vec![*expr])
            }
        }
    }

}


