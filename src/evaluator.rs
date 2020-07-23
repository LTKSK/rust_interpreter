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

fn eval_expression(expression: ast::Expression) -> Result<Object, EvalError> {
    match expression {
        ast::Expression::Integer(i) => Ok(Object::Integer(i)),
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

fn eval(program: ast::Program) -> Result<Object, EvalError> {
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
        let tests = vec![("5", 5), ("10", 10)];
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
}
