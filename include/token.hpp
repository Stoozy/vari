#pragma once

#include <string>
#include <variant>

using std::string;

static const string g_token_debug[37] =  {
    "LEFT_PAREN","RIGHT_PAREN","LEFT_BRACE","RIGHT_BRACE",
    "COMMA","DOT","MINUS","PLUS","SEMICOLON", "SLASH","STAR",

    // One or two character tokens.
    "NOT","NE",
    "EQ","DOUBLE_EQ",
    "GT","GEQ",
    "LT","LEQ",

    // Literals.
    "IDENTIFIER", "STRING","NUMBER",

    // Keywords.
    "AND","ELSE","FALSE","FUN","FOR","IF","NIL","OR",
    "PRINT","RETURN","THIS","TRUE","LET","WHILE",
    "EOF"

};


enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    NOT, NE,
    EQ, DOUBLE_EQ,
    GT, GEQ,
    LT, LEQ,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, THIS, TRUE, LET, WHILE,
    _EOF
};



typedef struct {
    const char * ptr;
    size_t len;
} cstr;

typedef union obj {
    uintptr_t ptr;
    int i32;
    double fp64;
    cstr str;
} Object;



class Token {
    string lexeme;
    Object literal;
    int line;

    public:
        enum TokenType type;
        Token();
        Token(enum TokenType type, string lexeme, Object literal, int line);
        ~Token();

        string toString();
};
