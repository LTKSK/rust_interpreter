use crate::ast;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EvalError {
    msg: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvalError: {}", self.msg)
    }
}

impl Error for EvalError {
    fn description(&self) -> &str {
        "Eval失敗"
    }
}

fn eval_prefix_bang_operator(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Boolean(b) => match b {
            true => Ok(Object::Boolean(false)),
            false => Ok(Object::Boolean(true)),
        },
        // NullはFalsyなのでひっくり返してtrue
        Object::Null => Ok(Object::Boolean(true)),
        // bool以外は全てtrulyな値として扱うので、返すのはfalse
        _ => Ok(Object::Boolean(false)),
    }
}

fn eval_prefix_minus_operator(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        _ => Ok(Object::Null),
    }
}

fn eval_prefix_expression(op: ast::PrefixOprator, right: Object) -> Result<Object, EvalError> {
    match op {
        ast::PrefixOprator::Bang => Ok(eval_prefix_bang_operator(right)?),
        ast::PrefixOprator::Minus => Ok(eval_prefix_minus_operator(right)?),
    }
}

fn eval_infix_expression(
    op: ast::InfixOprator,
    left: Object,
    right: Object,
) -> Result<Object, EvalError> {
    match op {
        ast::InfixOprator::Plus => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
            _ => Ok(Object::Null),
        },
        ast::InfixOprator::Minus => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
            _ => Ok(Object::Null),
        },
        ast::InfixOprator::Asterisk => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
            _ => Ok(Object::Null),
        },
        ast::InfixOprator::Slash => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
            _ => Ok(Object::Null),
        },
        o => panic!(
            "eval infix expression for {:?} is not unimplemented yet.",
            o
        ),
    }
}

fn eval_expression(expression: ast::Expression) -> Result<Object, EvalError> {
    match expression {
        ast::Expression::Integer(i) => Ok(Object::Integer(i)),
        ast::Expression::Bool(b) => Ok(Object::Boolean(b)),
        ast::Expression::Prefix { operator, right } => {
            let right = eval_expression(right.as_ref().clone())?;
            Ok(eval_prefix_expression(operator, right)?)
        }
        ast::Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(left.as_ref().clone())?;
            let right = eval_expression(right.as_ref().clone())?;
            Ok(eval_infix_expression(operator, left, right)?)
        }
        s => Err(EvalError {
            msg: format!("Unexpected Expression {:?}", s),
        }),
    }
}

fn eval_statement(statement: ast::Statement) -> Result<Object, EvalError> {
    match statement {
        ast::Statement::Expression(e) => Ok(eval_expression(e)?),
        s => Err(EvalError {
            msg: format!("Unexpected Statement {:?}", s),
        }),
    }
}

pub fn eval(program: ast::Program) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for stmt in program.statements {
        result = eval_statement(stmt)?;
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_integer() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-10", -10),
            ("5+5+5+5-10", 10),
            ("2 * 2* 2*2*2", 32),
            ("-50 + 100 + -50", 0),
            ("5*2 + 10", 20),
            ("5+2 * 10", 25),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5+10)", 30),
            ("3 + (5+10) * 2", 33),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            match eval(program) {
                Ok(o) => match o {
                    Object::Integer(i) => assert_eq!(i, expect),
                    o => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn test_eval_boolean() {
        let tests = vec![("true", true), ("false", false)];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            match eval(program) {
                Ok(o) => match o {
                    Object::Boolean(b) => assert_eq!(b, expect),
                    o => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn test_eval_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!!true", true),
            ("!!65", true),
            ("!5", false),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            match eval(program) {
                Ok(o) => match o {
                    Object::Boolean(b) => assert_eq!(b, expect),
                    o => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
