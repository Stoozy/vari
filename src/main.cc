#include <iostream>
#include <fstream>
#include <sstream>

#include <lexer.hpp>
#include <vari.hpp>

using std::cout; 
using std::cin; 
using std::string;

Vari g_vari;


void Vari::run(string source){

    //for(string token: tokens)
    //    cout << token;

    if(errored) exit(1);
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


void Vari::report(int line, string where, string msg){
    cout << "Error On line: " << line << " " << where << ": " << msg;
}

void Vari::error(int line, string msg){
    report(line, "", msg);
    errored = true;
}

int main(int argc, char * argv[]){

    if(argc > 2){
        cout << "Usage: vari <script>";
    } else if(argc == 2) {
        g_vari.run_file(argv[1]);
    } else {
        g_vari.run_prompt();
    }

    return 0;
}
