use std::io::{stdin, stdout};
mod ast;
mod lexer;
mod parser;
mod repr;
mod token;
use repr::start;

fn main() {
    start(stdin(), stdout());
}
