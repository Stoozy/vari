#pragma once

#include <string>

using std::string;

class Vari {
    bool errored = false;

    private:
        void run(string source);
        void report(int line, string where, string mesg);
    public:

        void error(int line, string mesg);
        void run_file(string filepath);
        void run_prompt();
};
