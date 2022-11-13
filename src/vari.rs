use crate::interpreter::Interpreter;
use crate::procedure::Procedure;
use crate::{lexer::Lexer, parser::Parser};

use std::collections::HashMap;
use std::io;
use std::{fs, io::Write};

#[derive(Debug, Clone)]
pub struct Vari {
    pub had_error: bool,
    pub interpreter: Interpreter,
}

pub enum VariError {
    Return(VariTypes),
}

#[derive(Debug, Clone)]
pub enum VariTypes {
    Nil,
    Num(f64),
    String(String),
    Boolean(bool),
    Struct(HashMap<String, VariTypes>),
    Callable(Procedure),
}

impl Vari {
    fn report(&self, line: usize, location: &str, msg: &str) -> () {
        println!("Error on line {}:  {} {}", line, location, msg);
    }

    fn run(&mut self, source: &str) {
        let mut lexer: Lexer = Lexer::new(source.to_owned(), (*self).clone());
        let tokens = lexer.scan_tokens();

        let mut parser: Parser = Parser::new(tokens);
        let statements = parser.parse();

        self.interpreter.interpret(statements);

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

    pub fn run_prompt(&mut self) -> () {
        loop {
            let mut user_inp = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut user_inp) {
                Ok(_) => {
                    self.run(user_inp.as_str());
                }
                Err(_) => todo!(),
            }
        }
    }

    pub fn run_file(&mut self, file_path: &str) {
        match self.read_source(file_path) {
            Ok(data) => {
                self.run(data.as_str());
            }
            Err(_) => todo!(),
        }
    }
}
