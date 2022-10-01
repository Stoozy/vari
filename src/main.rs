mod lexer;
mod token;
mod vari;
mod expr;
mod tests;
mod parser;

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
