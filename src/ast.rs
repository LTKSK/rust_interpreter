use crate::token::*;

#[derive(Clone, Debug)]
enum Statement {
    LetStatement(LetStatement),
}

#[derive(Clone, Debug)]
enum Expression {
    Identifier(Identifier),
}

#[derive(Clone, Debug)]
pub struct Program {
    statements: Vec<Statement>,
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
    name: String,
    value: Expression,
}

impl LetStatement {
    pub fn statement_node(&self) {}
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
    pub fn expression_node(&self) {}
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
