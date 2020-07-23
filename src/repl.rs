use crate::evaluator::*;
use crate::lexer::*;
use crate::parser::*;
use std::io::{Stdin, Stdout};

pub fn start(input: Stdin, output: Stdout) {
    loop {
        let mut s = String::new();
        input.read_line(&mut s).ok();
        if s == "exit".to_string() {
            break;
        }
        let mut l = Lexer::new(&s);
        let mut parser = Parser::new(&mut l);
        let program = parser.parse_program();
        match program {
            Ok(p) => {
                if let Ok(o) = eval(p) {
                    println!("> {}", o);
                }
            }
            Err(e) => println!("> {}", e),
        }
    }
}
