use crate::lexer::*;
use crate::token::Token;
use std::io::{Stdin, Stdout};

pub fn start(input: Stdin, output: Stdout) {
    let mut s = String::new();
    loop {
        input.read_line(&mut s).ok();
        if s == "exit".to_string() {
            break;
        }
        let mut l = Lexer::new(&s);
        loop {
            let t = l.next_token();
            if t == Token::EOF {
                break;
            }
            println!("{:?}", t);
        }
    }
}
