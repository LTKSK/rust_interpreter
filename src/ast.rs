use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InfixOprator {
    Plus,
    Minus,
    Slash,
    Asterisk,
    Gt,
    Lt,
    Assign,
    Equal,
    Nequal,
    Lparen, //関数呼び出しはLparenをinfixとして捉える
}

impl fmt::Display for InfixOprator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Slash => write!(f, "/"),
            Self::Asterisk => write!(f, "*"),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Assign => write!(f, "="),
            Self::Equal => write!(f, "=="),
            Self::Nequal => write!(f, "!="),
            Self::Lparen => write!(f, "("),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Expression {
    Identifier(String),
    Integer(i32),
    Bool(bool),
    String(String),
    Array(Vec<Expression>),
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
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    For {
        parameter: String,
        array: Box<Expression>, //Expression::Array only
        statement: Box<Statement>,
    },
    Map(BTreeMap<Box<Expression>, Box<Expression>>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(i) => write!(f, "{}", &i),
            Self::Integer(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Prefix { operator, right } => write!(f, "{}{}", operator, right),
            Self::Infix {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Self::Call {
                function,
                arguments,
            } => write!(
                f,
                "{}({})",
                &function,
                arguments
                    .iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Index { left, index } => write!(f, "{}[{}]", left, index),
            Self::For {
                parameter,
                array,
                statement,
            } => write!(f, "for {} in {} {{ {} }}", parameter, array, statement),
            Self::Array(exprs) => write!(
                f,
                "[{}]",
                exprs
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Map(m) => write!(
                f,
                "{{ {} }}",
                m.iter()
                    .map(|(k, v)| { format!("{}: {}", k, v) })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => write!(f, "todo exp {:?}", self),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Statement {
    // nameが変数名で、valueが=の右辺
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt).unwrap();
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Let { name, value } => write!(f, "let {} = {};", name, value),
            Self::Return(e) => write!(f, "return {};", e),
            Self::Expression(e) => write!(f, "{}", e),
            Self::Block(stmts) => {
                for s in stmts.iter() {
                    write!(f, "{}", s).unwrap();
                }
                Ok(())
            }
        }
    }
}
