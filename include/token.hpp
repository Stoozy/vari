#pragma once

#include <string>
#include <variant>

using std::string;


enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    NOT, NE,
    EQ, DOUBLE_EQ,
    GE, GEQ,
    LE, LEQ,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,
    _EOF
};



typedef struct {
    char * ptr;
    size_t len;
} cstr;

typedef union obj {
    uintptr_t ptr;
    int i32;
    cstr str;
} Object;



class Token {
    enum TokenType type;
    string lexeme;
    Object literal;
    int line;

    public:
        Token();
        Token(enum TokenType type, string lexeme, Object literal, int line);
        ~Token();

        string toString();
};
