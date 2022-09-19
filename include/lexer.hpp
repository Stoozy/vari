#pragma once

#include <string>
#include <vector>

#include "token.hpp"

using std::string;
using std::vector;

class Lexer {

    private:
        int start, current, line;

        void scanToken();
        char peek();
        char next();
        void addToken(TokenType type);
        void addToken(TokenType type, Object literal);

        bool match(char  c);
    public:
        string source;
        vector<Token> tokens;

        Lexer(string src);
        ~Lexer();

        vector<Token> scanTokens();

};
