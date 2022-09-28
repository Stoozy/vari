use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::{
    vari::token::{Token, TokenType},
    VARI,
};

pub struct Lexer {
    tokens: Vec<Token>,
    source: String,

    line: usize,
    current: usize,
    start: usize,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            tokens: vec![],
            source: src,
            line: 1,
            current: 0,
            start: 0,
        }
    }

    fn done(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }

    fn add_token_with_literal(&mut self, tk_type: TokenType, literal: Option<Arc<dyn Any>>) {
        let strval = self.source[self.start..self.current].to_owned();
        self.tokens
            .push(Token::new(tk_type, strval, self.line, literal));
    }

    fn add_token(&mut self, tk_type: TokenType) -> () {
        self.add_token_with_literal(tk_type, None);
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.as_bytes()[self.current + 1] as char
    }

    fn peek(&self) -> char {
        if self.done() {
            return '\0';
        }

        self.source.as_bytes()[self.current] as char
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn match_expected(&mut self, expected: char) -> bool {
        if self.done() || self.source.as_bytes()[self.current] != expected as u8 {
            return false;
        }

        self.current += 1;
        true
    }

    fn consume_num_literal(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num_substr = self.source[self.start..self.current].to_owned();

        match num_substr.parse::<f64>() {
            Ok(val) => {
                self.add_token_with_literal(TokenType::NUMBER, Some(Arc::new(val)));
            }
            _ => {
                VARI.error(self.line, "Invalid number type");
            }
        }
    }

    fn consume_string_literal(&mut self) {
        while self.peek() != '"' && !self.done() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        // closing "
        self.advance();

        let token_literal_value: String = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token_with_literal(TokenType::STRING, Some(Arc::new(token_literal_value)));
    }

    fn scan_token(&mut self) -> () {
        match self.advance() {
            '(' => self.add_token(TokenType::LPAREN),
            ')' => self.add_token(TokenType::RPAREN),
            '{' => self.add_token(TokenType::LBRACE),
            '}' => self.add_token(TokenType::RBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '*' => self.add_token(TokenType::STAR),
            '/' => self.add_token(TokenType::SLASH),
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            ';' => self.add_token(TokenType::SEMICOLON),

            // ignore whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,

            // operators
            '=' => {
                if self.match_expected('=') {
                    self.add_token(TokenType::ISEQ);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '!' => {
                if self.match_expected('=') {
                    self.add_token(TokenType::NE);
                } else {
                    self.add_token(TokenType::NOT);
                }
            }
            '<' => {
                if self.match_expected('=') {
                    self.add_token(TokenType::LE);
                } else {
                    self.add_token(TokenType::LT);
                }
            }
            '>' => {
                if self.match_expected('=') {
                    self.add_token(TokenType::GE);
                } else {
                    self.add_token(TokenType::GT);
                }
            }

            // comments
            '#' => {
                while self.peek() != '\n' && !self.done() {
                    self.advance();
                }
            }

            '"' => {
                self.consume_string_literal();
            }
            c => {
                if self.is_digit(c) {
                    self.consume_num_literal();
                } else {
                    VARI.error(self.line, format!("Unexpected character").as_str());
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_owned(),
            line: self.line,
            literal: None,
        });
        self.tokens.clone()
    }
}
