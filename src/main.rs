use std::io::{stdin, stdout};
mod lexer;
mod repr;
use repr::start;

fn main() {
    start(stdin(), stdout());
}
