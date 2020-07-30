use crate::ast;
use crate::builtins;
use crate::environment;
use crate::object::Object;
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

fn eval_prefix_bang_operator(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Boolean(b) => match b {
            true => Ok(Object::Boolean(false)),
            false => Ok(Object::Boolean(true)),
        },
        // NullはFalsyなのでひっくり返してtrue
        Object::Null => Ok(Object::Boolean(true)),
        // bool以外は全てtruthyな値として扱うので、返すのはfalse
        _ => Ok(Object::Boolean(false)),
    }
}

fn eval_prefix_minus_operator(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        _ => Err(EvalError {
            msg: "Invalid prefix expression".to_string(),
        }),
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
            (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Minus => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Asterisk => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Slash => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Gt => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l > r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Lt => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l < r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Equal => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l == r)),
            (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l == r)),
            (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l == r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        ast::InfixOprator::Nequal => match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l != r)),
            (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l != r)),
            (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l != r)),
            _ => Err(EvalError {
                msg: "Invalid infix expression".to_string(),
            }),
        },
        o => panic!("eval infix expression for {:?} is not implemented yet.", o),
    }
}

// parametersをkeyとしてargsで渡されたObjectをenv登録
fn extend_function_env(
    parameters: Vec<ast::Expression>,
    env: &environment::Environment,
    args: Vec<Object>,
) -> environment::Environment {
    let mut env = environment::Environment::new_enclosed(env);
    for (p, arg) in parameters.iter().zip(args) {
        if let ast::Expression::Identifier(i) = p {
            env.set(i.to_string(), arg)
        }
    }
    env
}

fn apply_function(function: Object, args: Vec<Object>) -> Result<Object, EvalError> {
    match function {
        Object::Function {
            parameters,
            body,
            env,
        } => {
            // parametersとargsの対応付け。関数の引数にあるparamsにargsのobjを対応させる
            let mut extended_env = extend_function_env(parameters, &env, args);
            let evaluated = eval_statement(body.as_ref().clone(), &mut extended_env)?;
            match evaluated {
                Object::Return(o) => Ok(o.as_ref().clone()),
                _ => Ok(evaluated),
            }
        }
        Object::Builtin(f) => match f(args) {
            Ok(result) => Ok(result),
            // Errorをenumにして表示側でmatchするとこんな変換をしなくてよくなる
            Err(e) => Err(EvalError { msg: e.msg }),
        },
        _ => Err(EvalError {
            msg: format!("{:?} Can not be called", function),
        }),
    }
}

fn eval_expressions(
    expressions: Vec<ast::Expression>,
    env: &mut environment::Environment,
) -> Result<Vec<Object>, EvalError> {
    let mut result = vec![];
    for e in expressions {
        let evaluated = eval_expression(e, env)?;
        result.push(evaluated);
    }
    Ok(result)
}

fn eval_expression(
    expression: ast::Expression,
    env: &mut environment::Environment,
) -> Result<Object, EvalError> {
    match expression {
        ast::Expression::Integer(i) => Ok(Object::Integer(i)),
        ast::Expression::Bool(b) => Ok(Object::Boolean(b)),
        ast::Expression::String(s) => Ok(Object::String(s)),
        ast::Expression::Prefix { operator, right } => {
            let right = eval_expression(right.as_ref().clone(), env)?;
            eval_prefix_expression(operator, right)
        }
        ast::Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(left.as_ref().clone(), env)?;
            let right = eval_expression(right.as_ref().clone(), env)?;
            eval_infix_expression(operator, left, right)
        }
        ast::Expression::If {
            condition,
            consequence,
            alternative,
        } => match eval_expression(condition.as_ref().clone(), env)? {
            Object::Boolean(b) => {
                if b {
                    eval_statement(consequence.as_ref().clone(), env)
                } else {
                    match alternative {
                        Some(a) => eval_statement(a.as_ref().clone(), env),
                        None => Ok(Object::Null),
                    }
                }
            }
            c => {
                return Err(EvalError {
                    msg: format!("If condition must be boolean, but got {:?}", c),
                })
            }
        },
        ast::Expression::Identifier(name) => {
            if let Some(o) = env.get(&name) {
                return Ok(o.clone());
            }

            // 都度生成は効率が悪い
            if let Some(o) = builtins::new().get(&name) {
                Ok(o.clone())
            } else {
                Err(EvalError {
                    msg: format!("Undefined variable {}", name),
                })
            }
        }
        ast::Expression::Call {
            function,
            arguments,
        } => {
            let args = eval_expressions(arguments, env)?;
            let function = eval_expression(function.as_ref().clone(), env)?;
            apply_function(function, args)
        }
        ast::Expression::Function { parameters, body } => Ok(Object::Function {
            parameters,
            body,
            env: env.clone(),
        }),
    }
}

fn eval_block_statements(
    statements: Vec<ast::Statement>,
    env: &mut environment::Environment,
) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for stmt in statements {
        result = eval_statement(stmt, env)?;
        if let Object::Return(_) = &result {
            return Ok(result);
        }
    }
    Ok(result)
}

fn eval_statement(
    statement: ast::Statement,
    env: &mut environment::Environment,
) -> Result<Object, EvalError> {
    match statement {
        ast::Statement::Expression(e) => eval_expression(e, env),
        ast::Statement::Block(statements) => eval_block_statements(statements, env),
        ast::Statement::Return(e) => Ok(Object::Return(Box::new(eval_expression(e, env)?))),
        ast::Statement::Let { name, value } => {
            let val = eval_expression(value, env)?;
            env.set(name, val.clone());
            Ok(val)
        }
    }
}

fn eval_statements(
    statements: Vec<ast::Statement>,
    env: &mut environment::Environment,
) -> Result<Object, EvalError> {
    let mut result = Object::Null;
    for stmt in statements {
        result = eval_statement(stmt, env)?;
        if let Object::Return(o) = result {
            return Ok(o.as_ref().clone());
        }
    }
    Ok(result)
}

pub fn eval(
    program: ast::Program,
    env: &mut environment::Environment,
) -> Result<Object, EvalError> {
    eval_statements(program.statements, env)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

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
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
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
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
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
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 > 1", false),
            ("1 < 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 11", false),
            ("1 != 11", true),
            ("true == true", true),
            ("false == false", true),
            ("false == true", false),
            ("false != true", true),
            ("true != false", true),
            ("(1<2) == false", false),
            ("(1>2) == false", true),
            ("(1>2) == true", false),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
                Ok(o) => match o {
                    Object::Boolean(b) => assert_eq!(b, expect),
                    o => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn test_eval_ifelse_expressions() {
        let tests = vec![
            ("if(true){10}", 10),
            ("if(false){10} else {20}", 20),
            ("if(false){10}", -1),
            ("if(1 > 0){10}", 10),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
                Ok(o) => match o {
                    Object::Integer(i) => assert_eq!(i, expect),
                    // Nullが返る分岐は-1をexpectとして入れておいて判定
                    // 数字は何でも良いんだけど、とりあえずこれで
                    Object::Null => assert_eq!(-1, expect),
                    _ => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn test_eval_return_expressions() {
        let tests = vec![
            ("return 10;9;", 10),
            ("return 2 * 5;", 10),
            ("9; return 5;", 5),
            ("if (true){ if (true) { return 10; } return 1;}", 10),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
                Ok(o) => match o {
                    Object::Integer(i) => assert_eq!(i, expect),
                    _ => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true", "EvalError: Invalid infix expression"),
            ("5 + true; 5", "EvalError: Invalid infix expression"),
            ("-true", "EvalError: Invalid prefix expression"),
            ("true+false", "EvalError: Invalid infix expression"),
            (
                "if(10>1){true + false;}",
                "EvalError: Invalid infix expression",
            ),
            (
                "if(10>1){if(10>1){true + false;} return 1; }",
                "EvalError: Invalid infix expression",
            ),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
                Ok(_) => panic!("expect error '{}'", expect),
                Err(e) => assert_eq!(format!("{}", e), expect),
            }
        }
    }

    #[test]
    fn test_call() {
        let tests = vec![
            ("let a = fn(b,c) {return b + c;}; a(10, 20)", 30),
            (
                "let f = fn(a){ fn(b){a+b};}; let c = f(10); let d = c(20);",
                30,
            ),
        ];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
                Ok(o) => match o {
                    Object::Integer(i) => assert_eq!(i, expect),
                    _ => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    #[test]
    fn test_builtin_function() {
        let tests = vec![(r#"len("")"#, 0), (r#"len("four")"#, 4)];
        for (input, expect) in tests {
            let mut l = Lexer::new(input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program().unwrap();
            let mut env = environment::Environment::new();
            match eval(program, &mut env) {
                Ok(o) => match o {
                    Object::Integer(i) => assert_eq!(i, expect),
                    _ => panic!("Error expect {} but got {:?}", expect, o),
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
