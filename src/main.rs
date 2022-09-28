pub mod lexer;
pub mod vari;

use std::env;
use std::fs;

//use vari::token::TokenType::*;
//use vari::token::Token;
use vari::vari::Vari;

use crate::lexer::Lexer;

const VARI: Vari = Vari { had_error: false };

impl Vari {
    fn report(&self, line: usize, location: &str, msg: &str) -> () {
        println!("Error on line {}:  {} {}", line, location, msg);
    }

    fn run(&self, source: &str) {
        println!("Source: {}", source);
        let mut lexer: Lexer = Lexer::new(source.to_owned());

        let tokens = lexer.scan_tokens();
        let n_tokens = tokens.len();

        for token in tokens {
            println!("{}", token.to_string());
        }

        println!("Scanned {} tokens", n_tokens);

        if self.had_error {
            std::process::exit(1);
        }
    }

    fn read_source(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(file_path)?;
        Ok(data)
    }

    pub fn error(&self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn run_prompt(&self) -> () {
        loop {}
    }

    pub fn run_file(&self, file_path: &str) {
        match self.read_source(file_path) {
            Ok(data) => {
                self.run(data.as_str());
            }
            Err(_) => todo!(),
        }
    }
}

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
