mod expr;
mod interpreter;
mod lexer;
mod parser;
mod tests;
mod token;
mod vari;

use std::env;
use vari::VARI;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: vari <file>");
    } else if args.len() == 2 {
        VARI.run_file(&args[1]);
    } else {
        VARI.run_prompt();
    }
}
