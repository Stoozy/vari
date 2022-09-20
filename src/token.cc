#include <token.hpp>
#include <algorithm>


Token::Token(){ }
Token::~Token(){ }

Token::Token(enum TokenType t, string lex, Object lit, int ln)
    : type(t), lexeme(lex), literal(lit), line(ln) { }


string Token::toString(){
    string ret = std::to_string(type);
    ret+= " " + lexeme + " ";

    switch(type){
        case IDENTIFIER:
        case STRING:

            for(size_t i=0; i<literal.str.len; i++)
                ret += literal.str.ptr[i];

            break;
        case NUMBER:
            ret += std::to_string(literal.i32);
            break;
        default:
            break;
    }

    return ret;
}
