mod environment;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod stmt;
mod tests;
mod token;
mod vari;

use interpreter::Interpreter;
use std::env;
use vari::Vari;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vari: Vari = Vari {
        had_error: false,
        interpreter: Interpreter::new(),
    };
    if args.len() > 2 {
        println!("Usage: vari <file>");
    } else if args.len() == 2 {
        vari.run_file(&args[1]);
    } else {
        vari.run_prompt();
    }
}
