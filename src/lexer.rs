use std::any::Any;

use crate::{vari::token::{TokenType, Token}, VARI};

pub struct Lexer {
    tokens : Vec<Token>,
    source:  String,

    line: usize,
    current: usize,
    start: usize
}

impl Lexer {

    pub fn new(src: String) -> Self {
        Self { 
            tokens: vec![], 
            source: src, 
            line:1, 
            current:0, 
            start:0
        }
    }

    fn done(&self) -> bool { self.current >= self.source.len() }

    fn advance(& mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current-1] as char
    }

    fn add_token_with_literal(&mut self, tk_type: TokenType, literal: Option<Box<dyn Any>>) {
        match literal {
            Some(_) => todo!(),
            None => {
                let text : String = self.source[self.start..self.current].to_owned();

                let new_token : Token =  Token {
                    token_type: tk_type, 
                    lexeme : text, 
                    line: self.line, 
                    literal: None
                };

                self.tokens.push(new_token);
            }
        }
        
    }

    fn add_token(&mut self, tk_type: TokenType) -> () { self.add_token_with_literal(tk_type, None); }


    fn scan_token(&mut self) -> () {
        match self.advance() {
            '(' => self.add_token(TokenType::LPAREN), 
            ')' => self.add_token(TokenType::RPAREN), 
            '{' => self.add_token(TokenType::LBRACE), 
            '}' => self.add_token(TokenType::RBRACE), 
            ',' => self.add_token(TokenType::COMMA), 
            '.' => self.add_token(TokenType::DOT), 
            '-' => self.add_token(TokenType::MINUS), 
            '+' => self.add_token(TokenType::PLUS), 
            ';' => self.add_token(TokenType::SEMICOLON), 
            '*' => self.add_token(TokenType::STAR),  

            // ignore whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line+=1,

            a => {
                println!("Got unknown char {:?}", a);
                VARI.error(self.line, "Unexpected character.");
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.done() {
            self.scan_token();
        }

        self.tokens.clone()
    }
}
