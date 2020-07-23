use std::io::{stdin, stdout};
mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repr;
mod token;
use repr::start;

fn main() {
    start(stdin(), stdout());
}
