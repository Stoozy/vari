#pragma once

#include <string>
#include <vector>

#include <map>
#include "token.hpp"

using std::string;
using std::vector;

class Lexer {

    private:
        std::map<string, int> keywords;
        int start, current, line;

        void scanToken();
        char peek();
        char peekNext();
        char next();
        void addToken(TokenType type);
        void addToken(TokenType type, Object literal);

        void consumeString();
        void consumeNumber();

        bool match(char  c);

        bool isAtEnd();
        bool isDigit(char c);
        bool isAlpha(char c);
        bool isAlphaNumeric(char c);
        
    public:

        string source;
        vector<Token> tokens;

        Lexer(string src);
        ~Lexer();

        vector<Token> scanTokens();

};
