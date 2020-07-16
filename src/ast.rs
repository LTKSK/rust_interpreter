use crate::token::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
}

#[derive(Clone, Debug)]
pub enum Expression {
    None, //Identifier(Identifier),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    // nameが変数名で、valueが=の右辺
    Let { name: Identifier, value: Expression },
    // tokenは自明なので保持しない
    Return { expression: Expression },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Let { name, value } => write!(f, "a"),
            Self::Return { expression } => write!(f, "b"),
        }
    }
}
