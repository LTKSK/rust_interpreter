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
    // nameが変数名で、valueが=の右辺
    Let { name: Identifier, value: Expression },
    // tokenは自明なので保持しない
    Return { expression: Expression },
}
