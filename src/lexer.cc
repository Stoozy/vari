#include <token.hpp>
#include <lexer.hpp>
#include <vari.hpp>

#include <iostream>

extern Vari g_vari;

bool Lexer::isAtEnd(){ return current >= source.size(); }

char Lexer::peek(){
   if (isAtEnd()) return '\0';
   return source[current];
}

char Lexer::peekNext(){
    if(current+1 >= source.size()) return '\0';
    return source[current+1];
}


bool Lexer::isDigit(char c){ return c >= '0' && c <= '9'; }

bool Lexer::isAlpha(char c){
    return (c >= 'a' && c <= 'z') || 
        (c >= 'A' && c <= 'Z') || c == '_';
}

bool Lexer::isAlphaNumeric(char c){ return isDigit(c) || isAlpha(c); }


char Lexer::next(){ 
    return source[current++]; 
}

// only conumes if current char is specifically c
bool Lexer::match(char c){
    if(isAtEnd()) return false;

    if(source[current] != c) return false;

    current++;
    return true;
}


void Lexer::consumeString(){
    while(peek() != '"' && !isAtEnd()){
        if(peek() == '\n') line++;
        next();
    }

    if(isAtEnd()){
        g_vari.error(line, "Nonterminated string.");
        return;
    }

    next(); // consumes closing quote

    string strval = source.substr(start+1, (current-start) - 1);

    cstr cstrval = {.ptr = strval.c_str(), .len=strval.size()};
    addToken(STRING, {.str = {.ptr=strval.c_str(), .len=strval.size()}});
}

void Lexer::consumeNumber(){
    while(isDigit(peek())) next();

    if(peek() == '.' && isDigit(peekNext())){
        next();

        while(isDigit(peek())) next();
    }

    addToken(NUMBER, {.fp64= std::stod(source.substr(start, current-start)) });
}

Lexer::Lexer(string src){
    start = 0; current = 0; line = 1; 
    source = src;

    keywords.insert({"and", AND});
    keywords.insert({"else", ELSE});
    keywords.insert({"false", FALSE});
    keywords.insert({"for", FOR});
    keywords.insert({"fun", FUN});
    keywords.insert({"if", IF});
    keywords.insert({"nil", NIL});
    keywords.insert({"or", OR});
    keywords.insert({"print", PRINT});
    keywords.insert({"return", RETURN});
    keywords.insert({"this", THIS});
    keywords.insert({"true", TRUE});
    keywords.insert({"let", LET});
    keywords.insert({"while", WHILE});
}

Lexer::~Lexer(){}


void Lexer::addToken(TokenType type,  Object literal){
    string text = source.substr(start, current-start);
    tokens.push_back(Token(type, text, literal, line));
}

void Lexer::addToken(TokenType type){ addToken(type, {.ptr=0}); }

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
        case '!': addToken(match('=') ? NE : NOT); break;
        case '=': addToken(match('=') ? DOUBLE_EQ : EQ); break;
        case '<': addToken(match('=') ? LEQ : LT); break;
        case '>': addToken(match('=') ? GEQ : GT); break;
        case '#':
            while(peek() != '\n') next(); 
            break;
        case '/' : addToken(SLASH); break;

        // other "pointless" chars
        case ' ':
        case '\r':
        case '\t':
            break;

        case  '\n': line++; break;
        case '"': consumeString(); break;

        default:
          if(isDigit(c)) 
              consumeNumber();
          else 
              g_vari.error(line, "Unexpected character.");
          break;
    }
}

vector<Token> Lexer::scanTokens(){ 
    while(!isAtEnd()){
        start = current;
        scanToken();
    }

    Object literal {.ptr=0};
    
    Token token(_EOF, "", literal,  line);
    tokens.push_back(token);

    return tokens;
}

