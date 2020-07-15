use crate::token::*;

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
    LetStatement { name: Identifier, value: Expression },
}
