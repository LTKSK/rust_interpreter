use crate::ast;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

fn eval(node: ast::Expression) -> Object {
    Object::Integer(5)
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
            let exp = match program.statements[0].clone() {
                ast::Statement::Expression(e) => e,
                _ => panic!("Error expected Statement::Expression"),
            };
            match eval(exp) {
                Object::Integer(i) => assert_eq!(i, expect),
                o => panic!("Error expect {} but got {:?}", expect, o),
            }
        }
    }
}
