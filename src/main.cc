#include <iostream>
#include <fstream>
#include <sstream>

using std::cout; 
using std::cin; 
using std::string;

#include <util.h>


class Vari{
    private:
        void run(string source);
        void report(int line, string where, string mesg);
    public:
        void error(int line, string mesg);
        void run_file(string filepath);
        void run_prompt();
};


void Vari::run(string source){
    vector<string> tokens = util::split(source, " ");

    for(string token: tokens)
        cout << token;
}

void Vari::run_file(string fp){
    std::ifstream input_file;
    input_file.open(fp);

    std::stringstream ss;
    ss << input_file.rdbuf();

    run(ss.str());
}

void Vari::run_prompt(){ 
    for(;;){
        string line;
        cout << "> ";
        cin >> line;
        if(line.size() == 0)
            break;
        run(line);
    }
}


int main(int argc, char * argv[]){
    Vari vari;

    if(argc > 2){
        cout << "Usage: vari <script>";
    } else if(argc == 2) {
        vari.run_file(argv[1]);
    } else {
        vari.run_prompt();
    }

    return 0;
}
