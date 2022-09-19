#include "token.hpp"
#include <lexer.hpp>
#include <vari.hpp>

extern Vari g_vari;

char Lexer::peek(){
   if (  current >= source.size()) return '\0';
   return source[current];
}

bool Lexer::match(char c){
    if( current >= source.size()) return false;

    if(source[current] != c) return false;

    current++;
    return true;
}

char Lexer::next(){
    return source[current++];
}

Lexer::Lexer(string src){
    start = 0; current = 0; line = 1; 
    source = src;
}

void Lexer::addToken(TokenType type){
    addToken(type, {.ptr=0});
}

void Lexer::addToken(TokenType type,  Object literal){
    string text = source.substr(start, current-start);
    tokens.push_back(Token(type, text, literal, line));
}

void Lexer::scanToken(){
    char c = next();
    switch(c){
        case '(': addToken(LEFT_PAREN); break;
        case ')': addToken(RIGHT_PAREN); break;
        case '{': addToken(LEFT_BRACE); break;
        case '}': addToken(RIGHT_BRACE); break;
        case ',': addToken(COMMA); break;
        case '.': addToken(DOT); break;
        case '-': addToken(MINUS); break;
        case '+': addToken(PLUS); break;
        case ';': addToken(SEMICOLON); break;
        case '*': addToken(STAR); break;

        // other operators
        case '!':
            addToken(match('=') ? NE : NOT);
            break;
        case '=':
            addToken(match('=') ? DOUBLE_EQ : EQ);
            break;
        case '<':
            addToken(match('=') ? LEQ : LE);
            break;
        case '>':
            addToken(match('=') ? GEQ : GE);
            break;
        case '#':
            while(peek() != '\n') next(); 
            break;
        case '/' :
            addToken(SLASH);
            break;

        // other "pointless" chars
        case ' ':
        case '\r':
        case '\t':
            break;
        case  '\n':
            line++;
            break;

        default:
          g_vari.error(line, "Unexpected character.");
          break;
    }
}

vector<Token> Lexer::scanTokens(){ 
    while(current < source.size()){
        start = current;
        scanToken();
    }

    Object literal {.ptr=0};
    
    Token token(_EOF, "", literal,  line);

    tokens.push_back(token);

    return tokens;
}

