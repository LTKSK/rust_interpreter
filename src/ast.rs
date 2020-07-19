use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrefixOprator {
    Minus,
    Bang,
}

impl fmt::Display for PrefixOprator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InfixOprator {
    Plus,
    Minus,
    Slash,
    Asterisk,
    Gt,
    Lt,
    Equal,
    Nequal,
}

impl fmt::Display for InfixOprator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "!"),
            Self::Minus => write!(f, "-"),
            Self::Slash => write!(f, "/"),
            Self::Asterisk => write!(f, "*"),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Equal => write!(f, "=="),
            Self::Nequal => write!(f, "!="),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i32),
    Bool(bool),
    Prefix {
        operator: PrefixOprator,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: InfixOprator,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(i) => write!(f, "{}", &i),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Prefix { operator, right } => write!(f, "{}{}", operator, right),
            Self::Infix {
                left,
                operator,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            _ => write!(f, "todo exp"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    // nameが変数名で、valueが=の右辺
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Let { name, .. } => write!(f, "let {} = ident;", name),
            Self::Return(e) => write!(f, "return {};", e),
            Self::Expression(e) => write!(f, "{}", e),
            Self::Block(stmts) => {
                for s in stmts.iter() {
                    write!(f, "{}", s);
                }
                Ok(())
            }
        }
    }
}
