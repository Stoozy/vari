use crate::vari::VariTypes;
use crate::{
    expr::Expr,
    token::{Token, TokenType},
    vari::VARI,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn consume(&mut self, token_type: TokenType, err: &str) {
        if self.check(token_type) {
            return self.advance();
        }

        VARI.error(0, err);
    }

    fn primary(&mut self) -> Expr {
        if self.match_list(vec![TokenType::FALSE]) {
            return Expr::Literal {
                value: Box::new(VariTypes::Boolean(false)),
            };
        }

        if self.match_list(vec![TokenType::TRUE]) {
            return Expr::Literal {
                value: Box::new(VariTypes::Boolean(true)),
            };
        }

        if self.match_list(vec![TokenType::NIL]) {
            return Expr::Literal {
                value: Box::new(VariTypes::Nil),
            };
        }

        if self.match_list(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal {
                value: self.prev_token().literal.unwrap(),
            };
        }

        if self.match_list(vec![TokenType::LPAREN]) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RPAREN, "Expect ')' after expression.");
            return Expr::Grouping {
                expr: Box::new(expr),
            };
        }

        panic!("Expected expression.");
    }

    fn unary(&mut self) -> Expr {
        if self.match_list(vec![TokenType::NOT, TokenType::MINUS]) {
            let operator: Token = self.prev_token();
            let rhs = self.unary();
            return Expr::Unary {
                op: operator,
                rhs: Box::new(rhs),
            };
        }

        return self.primary();
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_list(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.prev_token();
            let rhs: Expr = self.unary();
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op: operator,
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_list(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.prev_token();
            let rhs: Expr = self.factor();
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op: operator,
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_list(vec![
            TokenType::GT,
            TokenType::GE,
            TokenType::LT,
            TokenType::LE,
        ]) {
            let operator: Token = self.prev_token();
            let rhs: Expr = self.term();
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op: operator,
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn advance(&mut self) {
        if !self.done() {
            self.current += 1;
        }
        self.prev_token();
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn prev_token(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn done(&self) -> bool {
        matches!(self.tokens[self.current].token_type, TokenType::EOF)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.done() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn match_list(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        while self.match_list(vec![TokenType::NE, TokenType::ISEQ]) {
            let operator: Token = self.prev_token();
            let rhs: Expr = self.comparison();
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op: operator,
                rhs: Box::new(rhs),
            };
        }

        expr
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
}
