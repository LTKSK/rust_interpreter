use crate::token::*;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrefixOprator {
    Minus,
    Bang,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i32),
    Prefix {
        operator: PrefixOprator,
        right: Box<Expression>,
    },
}

//impl fmt::Display for Expression
#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    // nameが変数名で、valueが=の右辺
    Let { name: String, value: Expression },
    Return(Expression),
    ExpressionStatement(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Let { name, .. } => write!(f, "{}", format!("let {} = ident;", name)),
            Self::Return(e) => write!(f, "{}", format!("return ident;")),
            Self::ExpressionStatement(e) => write!(f, "{}", format!("{:?}", e)),
        }
    }
}
