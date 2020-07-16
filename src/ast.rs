use crate::token::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(String),
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
    Return(Expression),
    ExpressionStatement(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Let { name, .. } => {
                write!(f, "{}", format!("let {} = ident;", name.token.literal))
            }
            Self::Return(e) => write!(f, "{}", format!("return ident;")),
            Self::ExpressionStatement(e) => write!(f, "{}", format!("{:?}", e)),
        }
    }
}
