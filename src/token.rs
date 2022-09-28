use std::any::Any;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    NOT,
    NE,
    EQUAL,
    ISEQ,
    GT,
    GE,
    LT,
    LE,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Arc<dyn Any>>,
}

impl Token {
    pub fn new(t: TokenType, lex: String, lno: usize, lit: Option<Arc<dyn Any>>) -> Self {
        Token {
            token_type: t,
            lexeme: lex,
            line: lno,
            literal: lit,
        }
    }

    pub fn to_string(self) -> String {
        format!(
            "{} {} {:?}",
            self.token_type.to_string(),
            self.lexeme,
            self.literal
        )
    }
}
