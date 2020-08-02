use crate::environment::Environment;
use crate::evaluator::*;
use crate::lexer::*;
use crate::parser::*;
use std::io::Stdin;

pub fn start(input: Stdin) {
    let mut env = Environment::new();
    println!("> Hello!");
    loop {
        let mut s = String::new();
        input.read_line(&mut s).ok();
        let mut l = Lexer::new(&s);
        let mut parser = Parser::new(&mut l);
        let program = parser.parse_program();
        match program {
            Ok(p) => match eval(p, &mut env) {
                Ok(result) => println!("> {}", result),
                Err(e) => println!("> {}", e),
            },
            Err(e) => {
                println!("> {}", e);
            }
        }
    }
}
