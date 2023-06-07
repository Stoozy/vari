#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, vari::Vari, interpreter::Interpreter};


    #[test]
    pub fn ast_test() {
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

    #[test]
    pub fn fib_lexer_test(){
        let source = "
        fun fib(n) {                    \
            if (n <= 1) return 1;       \
            return fib(n-1) + fib(n-2); \
        }                               \
                                        \
        for (let i=0; i<10; i = i+1){   \
            print fib(i);               \
        }";

        let vari: Vari = Vari {
            had_error: false,
            interpreter: Interpreter::new(),
        };

        let mut lexer = Lexer::new(String::from(source), vari);

        let result = "IDENTIFIER fun None IDENTIFIER fib None LPAREN ( None IDENTIFIER n None RPAREN ) None LBRACE { None IF if None LPAREN ( None IDENTIFIER n None LE <= None NUMBER 1 Some(Num(1.0)) RPAREN ) None RETURN return None NUMBER 1 Some(Num(1.0)) SEMICOLON ; None RETURN return None IDENTIFIER fib None LPAREN ( None IDENTIFIER n None MINUS - None NUMBER 1 Some(Num(1.0)) RPAREN ) None PLUS + None IDENTIFIER fib None LPAREN ( None IDENTIFIER n None MINUS - None NUMBER 2 Some(Num(2.0)) RPAREN ) None SEMICOLON ; None RBRACE } None FOR for None LPAREN ( None LET let None IDENTIFIER i None EQUAL = None NUMBER 0 Some(Num(0.0)) SEMICOLON ; None IDENTIFIER i None LT < None NUMBER 10 Some(Num(10.0)) SEMICOLON ; None IDENTIFIER i None EQUAL = None IDENTIFIER i None PLUS + None NUMBER 1 Some(Num(1.0)) RPAREN ) None LBRACE { None PRINT print None IDENTIFIER fib None LPAREN ( None IDENTIFIER i None RPAREN ) None SEMICOLON ; None RBRACE } None EOF  None ";


        let tokens = lexer.scan_tokens();
        let mut tokenstr = String::new();
        for token in tokens {
            tokenstr.push_str(format!("{} ", token.clone().to_string()).as_str());
        }
        assert!(tokenstr == result);
    }

}
