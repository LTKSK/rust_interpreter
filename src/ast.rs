use crate::token::*;

#[derive(Clone, Debug)]
pub enum Statement {
    LetStatement(LetStatement),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match &self {
            Statement::LetStatement(s) => s.token_literal(),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements[0] {
                Statement::LetStatement(s) => s.token_literal(),
                _ => unreachable!(),
            }
        } else {
            "".to_string()
        }
    }
}

#[derive(Clone, Debug)]
struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl LetStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Clone, Debug)]
struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
