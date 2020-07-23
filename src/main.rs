use std::io::{stdin, stdout};
mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;
use repl::start;

fn main() {
    start(stdin(), stdout());
}
