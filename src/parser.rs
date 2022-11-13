use std::collections::HashMap;

use crate::stmt::Stmt;
use crate::vari::VariTypes;
use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn consume(&mut self, token_type: TokenType, err: &str) -> Token {
        if self.check(token_type) {
            self.advance();
            return self.tokens[self.current - 1].clone();
        }

        panic!("Error: {}", err)
        //VARI.error(0, err); // maybe make vari a member?
        //unreachable!()
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

        if self.match_list(vec![TokenType::IDENTIFIER]) {
            return Expr::Variable {
                value: self.prev_token(),
            };
        }

        if self.match_list(vec![TokenType::LPAREN]) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RPAREN, "Expect ')' after expression.");
            return Expr::Grouping {
                expr: Box::new(expr),
            };
        }

        if self.match_list(vec![TokenType::LBRACE]) {
            let mut props = HashMap::new();

            while !self.check(TokenType::RBRACE) {
                let name = self.consume(TokenType::IDENTIFIER, "Expected name for property");
                self.consume(TokenType::COLON, "Expected colon after property name");
                let expr = self.expression();

                props.insert(name.lexeme, expr);

                match self.peek().token_type {
                    TokenType::RBRACE => {
                        self.consume(TokenType::RBRACE, "Expected '}' after struct");
                        return Expr::Struct { values: props };
                    }
                    TokenType::COMMA => {
                        self.consume(TokenType::COMMA, "Expected ',' after field expression");
                    }
                    _ => {
                        panic!("Unexpected token."); // TODO: handle this properly
                    }
                }
            }

            // only happens when struct is empty
            self.consume(TokenType::RBRACE, "Expected '}' after struct");
            return Expr::Struct { values: props };
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

        return self.call();
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

        while self.match_list(vec![TokenType::MINUS, TokenType::PLUS, TokenType::MODULO]) {
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
        self.assignment()
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

    // checks if next token is `token_type`
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

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.check(TokenType::RBRACE) && !self.done() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RBRACE, "Expect '}' after block.");

        statements
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.match_list(vec![TokenType::EQUAL]) {
            let _equals = self.prev_token();
            let rhs = self.assignment();

            if let Expr::Variable { value } = expr {
                return Expr::Assign {
                    name: value,
                    value_expr: Box::new(rhs),
                };
            } else if let Expr::Get { expr, name } = expr {
                return Expr::Set {
                    expr,
                    name,
                    value: Box::new(rhs),
                };
            }

            todo!()
            // error : "Invalid assignment" on token equals
        }

        return expr;
    }

    fn and(&mut self) -> Expr {
        let mut lhs = self.equality();

        while self.match_list(vec![TokenType::AND]) {
            let operator = self.prev_token();
            let rhs = self.equality();
            lhs = Expr::Logical {
                lhs: Box::new(lhs),
                operator,
                rhs: Box::new(rhs),
            }
        }

        return lhs;
    }

    fn or(&mut self) -> Expr {
        let lhs: Expr = self.and();

        while self.match_list(vec![TokenType::OR]) {
            let operator = self.prev_token();
            let rhs = self.and();
            return Expr::Logical {
                lhs: Box::new(lhs),
                operator,
                rhs: Box::new(rhs),
            };
        }

        return lhs;
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

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut args = vec![];
        if !self.check(TokenType::RPAREN) {
            args.push(self.expression());
            while self.match_list(vec![TokenType::COMMA]) {
                args.push(self.expression());
            }
        }

        let paren = self.consume(TokenType::RPAREN, "Expected ')' after arguments.");

        if args.len() > 255 {
            // TODO: handle error
            panic!("Can't have more than 255 arguments.");
        }

        Expr::Call {
            callee: Box::new(callee),
            paren,
            args,
        }
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            if self.match_list(vec![TokenType::LPAREN]) {
                expr = self.finish_call(expr);
            } else if self.match_list(vec![TokenType::DOT]) {
                let name = self.consume(TokenType::IDENTIFIER, "Expected property name after '.'.");
                expr = Expr::Get {
                    expr: Box::new(expr),
                    name: name.lexeme,
                };
            } else {
                break;
            }
        }

        expr
    }

    fn for_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LPAREN, "Expect  '(' after 'for'.");

        let initializer: Stmt;

        if self.match_list(vec![TokenType::SEMICOLON]) {
            initializer = Stmt::Expression(Expr::Literal {
                value: Box::new(VariTypes::Nil),
            });
        } else if self.match_list(vec![TokenType::LET]) {
            initializer = self.var_decl();
        } else {
            initializer = self.expr_stmt();
        }

        let mut condition: Option<Expr> = None;

        if !self.check(TokenType::SEMICOLON) {
            condition = Some(self.expression());
        }

        self.consume(
            TokenType::SEMICOLON,
            "Expect  ';' after for loop condition.",
        );

        let mut modifying_expr: Option<Expr> = None;
        if !self.check(TokenType::RPAREN) {
            modifying_expr = Some(self.expression());
        }

        self.consume(TokenType::RPAREN, "Expect  ')' after for loop condition.");

        let mut body = self.statement();

        if let Some(mod_expr) = modifying_expr {
            body = Stmt::Block(vec![body, Stmt::Expression(mod_expr)]);
        }

        if let Some(cond) = condition {
            body = Stmt::While(cond, Box::new(body));
        } else {
            body = Stmt::While(
                Expr::Literal {
                    value: Box::new(VariTypes::Boolean(true)),
                },
                Box::new(body),
            );
        }

        body = Stmt::Block(vec![initializer, body]);
        body
    }

    fn if_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LPAREN, "Expect  '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RPAREN, "Expect  ')' after if condition.");

        let then_block = Box::new(self.statement());
        let mut else_block = None;

        if self.match_list(vec![TokenType::ELSE]) {
            else_block = Some(Box::new(self.statement()));
        }

        return Stmt::If(condition, then_block, else_block);
    }

    fn return_stmt(&mut self) -> Stmt {
        let keyword = self.prev_token();
        let mut value = Expr::Literal {
            value: Box::new(VariTypes::Nil),
        };

        if !self.check(TokenType::SEMICOLON) {
            value = self.expression();
        }

        self.consume(TokenType::SEMICOLON, "Expect ';' after return value");

        Stmt::Return(keyword, value)
    }

    fn while_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LPAREN, "Expect '(' after 'while'");
        let condition = self.expression();
        self.consume(TokenType::RPAREN, "Expect ')' after 'while'");

        let body = self.statement();

        Stmt::While(condition, Box::new(body))
    }

    fn print_stmt(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::SEMICOLON, "Expected ';' after value.");
        Stmt::Print(value)
    }

    fn expr_stmt(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expected ';' after value.");
        Stmt::Expression(expr)
    }

    fn fun_decl(&mut self) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, "Expected function name after 'fun'");
        self.consume(TokenType::LPAREN, "Expected '(' after function name.");

        let mut params = vec![];

        if !self.check(TokenType::RPAREN) {
            params.push(self.consume(TokenType::IDENTIFIER, "Expected parameter name"));
            while self.match_list(vec![TokenType::COMMA]) {
                params.push(self.consume(TokenType::IDENTIFIER, "Expected parameter name"));
            }
        }

        self.consume(TokenType::RPAREN, "Expected ')' after parameters.");
        self.consume(TokenType::LBRACE, "Expected '{' after arguments list.");
        let body = self.block();

        Stmt::Function(name, params, body)
    }

    fn var_decl(&mut self) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, "Expected variable name.");

        let mut initializer_expr: Option<Expr> = None;

        if self.match_list(vec![TokenType::EQUAL]) {
            initializer_expr = Some(self.expression());
        }

        self.consume(TokenType::SEMICOLON, "Expected ';' after value.");
        return Stmt::Var(name, initializer_expr);
    }

    fn struct_decl(&mut self) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, "Expected struct name.");
        let mut initializer_expr: Option<Expr> = None;

        if self.match_list(vec![TokenType::EQUAL]) {
            initializer_expr = Some(self.expression()); // this should be Expr::Struct
        }

        self.consume(TokenType::SEMICOLON, "Expected ';' after value.");
        return Stmt::Var(name, initializer_expr);
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_list(vec![TokenType::FUN]) {
            return self.fun_decl();
        }

        if self.match_list(vec![TokenType::LET]) {
            return self.var_decl();
        }

        if self.match_list(vec![TokenType::STRUCT]) {
            return self.struct_decl();
        }

        return self.statement();
    }

    fn statement(&mut self) -> Stmt {
        if self.match_list(vec![TokenType::PRINT]) {
            return self.print_stmt();
        }

        if self.match_list(vec![TokenType::RETURN]) {
            return self.return_stmt();
        }

        if self.match_list(vec![TokenType::LBRACE]) {
            return Stmt::Block(self.block());
        }

        if self.match_list(vec![TokenType::WHILE]) {
            return self.while_stmt();
        }

        if self.match_list(vec![TokenType::FOR]) {
            return self.for_stmt();
        }

        if self.match_list(vec![TokenType::IF]) {
            return self.if_stmt();
        }

        self.expr_stmt()
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.done() {
            statements.push(self.declaration());
        }

        statements
    }
}
