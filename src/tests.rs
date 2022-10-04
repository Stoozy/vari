#[cfg(test)]
mod Tests {

    #[test]
    pub fn lexer_test() {
        use crate::expr::AstPrinter;
        use crate::expr::Expr;
        use crate::token::{Token, TokenType};
        use crate::vari::VariTypes;

        let mut printer: AstPrinter = AstPrinter::new();

        let expr = Expr::Binary {
            lhs: Box::new(Expr::Unary {
                op: Token {
                    token_type: TokenType::MINUS,
                    lexeme: "-".to_owned(),
                    literal: None,
                    line: 1,
                },
                rhs: Box::new(Expr::Literal {
                    value: Box::new(VariTypes::Num(25.0)),
                }),
            }),
            op: Token {
                token_type: TokenType::STAR,
                lexeme: "*".to_owned(),
                literal: None,
                line: 1,
            },
            rhs: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Literal {
                    value: Box::new(VariTypes::Num(45.67)),
                }),
            }),
        };

        assert!("( * ( - 25) ( group 45.67))" == printer.print(expr));
    }
}
