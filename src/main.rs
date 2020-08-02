use std::io::{stdin, stdout};
mod ast;
mod builtins;
mod environment;
mod error;
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
