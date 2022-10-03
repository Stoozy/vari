use crate::{
    expr::{AstPrinter, Expr, Interpreter},
    lexer::Lexer,
    parser::{self, Parser},
};

use std::io;
use std::{any::Any, sync::Arc};
use std::{
    fs,
    io::{stdout, Write},
};

pub static VARI: Vari = Vari { had_error: false };

pub struct Vari {
    pub had_error: bool,
}

#[derive(Debug, Clone)]
pub enum VariTypes {
    Nil,
    Num(f64),
    String(String),
    Boolean(bool),
    Object(Arc<dyn Any>),
}

impl Vari {
    fn report(&self, line: usize, location: &str, msg: &str) -> () {
        println!("Error on line {}:  {} {}", line, location, msg);
    }

    fn run(&self, source: &str) {
        println!("Source: {}", source);
        let mut lexer: Lexer = Lexer::new(source.to_owned());

        let tokens = lexer.scan_tokens();
        let mut parser: Parser = Parser::new(tokens);

        let expression: Expr = parser.parse();

        let interpreter: Interpreter = Interpreter::new();
        interpreter.interpret(expression);

        //for token in tokens {
        //    println!("{}", token.to_string());
        //}

        if self.had_error {
            std::process::exit(1);
        }

        //let mut printer : AstPrinter = AstPrinter::new();
        //println!("{}", printer.print(expression));
    }

    fn read_source(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(file_path)?;
        Ok(data)
    }

    pub fn error(&self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn run_prompt(&self) -> () {
        loop {
            let mut user_inp = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut user_inp) {
                Ok(val) => {
                    self.run(user_inp.as_str());
                }
                Err(_) => todo!(),
            }
        }
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
