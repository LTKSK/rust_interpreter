use crate::ast;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // ==
    Lessgreater, // > <
    Sum,         // + -
    Product,     // * /
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl Precedence {
    fn FromTokenKind(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::EQ | TokenKind::NEQ => Self::Equals,
            TokenKind::LT | TokenKind::GT => Self::Lessgreater,
            TokenKind::PLUS | TokenKind::MINUS => Self::Sum,
            TokenKind::SLASH | TokenKind::ASTERISK => Self::Product,
            _ => Self::Lowest,
        }
    }
}

#[derive(Debug)]
struct ParseError {
    msg: String,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ParseError!")
    }
}
impl Error for ParseError {
    fn description(&self) -> &str {
        "Parse失敗！"
    }
}

#[derive(Debug)]
struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lexer: lexer,
            current_token: Token {
                kind: TokenKind::ILLEGAL,
                literal: "".to_string(),
            },
            peek_token: Token {
                kind: TokenKind::ILLEGAL,
                literal: "".to_string(),
            },
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::FromTokenKind(&self.peek_token.kind)
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            return true;
        }
        return false;
    }

    pub fn parse_let_statement(&mut self) -> Result<ast::Statement, ParseError> {
        if !self.expect_peek(TokenKind::IDENT) {
            return Err(ParseError {
                msg: format!(
                    "expect `{}` but got `{}`",
                    TokenKind::IDENT,
                    self.peek_token.kind,
                ),
            });
        }
        let identifier = self.current_token.clone().literal;
        if !self.expect_peek(TokenKind::ASSIGN) {
            return Err(ParseError {
                msg: format!(
                    "expect `{}` but got `{}`",
                    TokenKind::IDENT,
                    self.peek_token.kind,
                ),
            });
        }
        // TODO: implelemnt expression
        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }
        let stmt = ast::Statement::Let {
            name: identifier,
            value: ast::Expression::Identifier("dummy".to_string()),
        };
        Ok(stmt)
    }

    pub fn parse_return_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let stmt = ast::Statement::Return(ast::Expression::Identifier("dummy".to_string()));
        self.next_token();
        // TODO: ここにExpressionのparse
        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }
        Ok(stmt)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression, ParseError> {
        let mut expression = self.parse_prefix()?;
        while !self.peek_token_is(TokenKind::SEMICOLON) && precedence < self.peek_precedence() {
            self.next_token();
            expression = self.parse_infix(expression)?;
        }
        Ok(expression)
    }

    fn parse_infix(&mut self, left: ast::Expression) -> Result<ast::Expression, ParseError> {
        let operator = match &self.current_token.kind {
            TokenKind::PLUS => ast::InfixOprator::Plus,
            TokenKind::MINUS => ast::InfixOprator::Minus,
            TokenKind::ASTERISK => ast::InfixOprator::Asterisk,
            TokenKind::SLASH => ast::InfixOprator::Slash,
            TokenKind::GT => ast::InfixOprator::Gt,
            TokenKind::LT => ast::InfixOprator::Lt,
            TokenKind::EQ => ast::InfixOprator::Equal,
            TokenKind::NEQ => ast::InfixOprator::Nequal,
            _ => return Ok(left),
        };

        let precedence = Precedence::FromTokenKind(&self.current_token.kind);
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(ast::Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression, ParseError> {
        match self.current_token.kind {
            TokenKind::IDENT => Ok(ast::Expression::Identifier(
                self.current_token.clone().literal,
            )),
            TokenKind::INT => Ok(ast::Expression::Integer(
                self.current_token.clone().literal.parse::<i32>().unwrap(),
            )),
            TokenKind::MINUS => {
                self.next_token();
                Ok(ast::Expression::Prefix {
                    operator: ast::PrefixOprator::Minus,
                    right: Box::new(self.parse_expression(Precedence::Prefix)?),
                })
            }
            TokenKind::BANG => {
                self.next_token();
                Ok(ast::Expression::Prefix {
                    operator: ast::PrefixOprator::Bang,
                    right: Box::new(self.parse_expression(Precedence::Prefix)?),
                })
            }
            _ => Err(ParseError {
                msg: "Unexpected Expression".to_string(),
            }),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }
        Ok(ast::Statement::ExpressionStatement(expression))
    }

    pub fn parse_statement(&mut self) -> Result<ast::Statement, ParseError> {
        match &self.current_token.kind {
            TokenKind::LET => Ok(self.parse_let_statement()?),
            TokenKind::RETURN => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { statements: vec![] };
        while self.current_token.kind != TokenKind::EOF {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        }
        Ok(program)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;"#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);
        let tests = vec!["let x = ident;", "let y = ident;", "let foobar = ident;"];
        for (index, stmt) in program.statements.iter().enumerate() {
            // TODO ident となっているところはexpressionを定義したら実装
            assert_eq!(format!("{}", stmt), tests[index]);
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
            return 5;
            return 10;
            return 993322;
            "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);
        let tests = vec!["return ident;", "return ident;", "return ident;"];
        for (index, stmt) in program.statements.iter().enumerate() {
            assert_eq!(format!("{}", stmt), tests[index]);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let ident = match &program.statements[0] {
            ast::Statement::ExpressionStatement(e) => match e {
                ast::Expression::Identifier(s) => s,
                _ => "unreach",
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
        assert_eq!(ident, "foobar");
    }

    #[test]
    fn test_int_expression() {
        let input = "5;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let ident = match &program.statements[0] {
            ast::Statement::ExpressionStatement(e) => match e {
                ast::Expression::Integer(i) => i,
                _ => &999,
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
        assert_eq!(ident, &5);
    }

    #[test]
    fn test_prefix_expression() {
        let input = r#"
            -5;
            !5;
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 2);

        let tests = [
            (ast::PrefixOprator::Minus, 5),
            (ast::PrefixOprator::Bang, 5),
        ];
        for (index, stmt) in program.statements.iter().enumerate() {
            let prefix = match stmt {
                ast::Statement::ExpressionStatement(e) => match e {
                    ast::Expression::Prefix { operator, right } => (
                        operator,
                        match right.as_ref() {
                            ast::Expression::Integer(i) => i,
                            _ => panic!("Invalid Right hand"),
                        },
                    ),
                    _ => panic!("Invalid Prefix Expression"),
                },
                e => panic!(format!("expect `Expression` but got {:?}", e),),
            };
            assert_eq!(*prefix.0, tests[index].0);
            assert_eq!(*prefix.1, tests[index].1);
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = r#"
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 < 5;
            5 > 5;
            5 == 5;
            5 != 5;
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 8);

        let tests = [
            (5, ast::InfixOprator::Plus, 5),
            (5, ast::InfixOprator::Minus, 5),
            (5, ast::InfixOprator::Asterisk, 5),
            (5, ast::InfixOprator::Slash, 5),
            (5, ast::InfixOprator::Lt, 5),
            (5, ast::InfixOprator::Gt, 5),
            (5, ast::InfixOprator::Equal, 5),
            (5, ast::InfixOprator::Nequal, 5),
        ];

        for (index, stmt) in program.statements.iter().enumerate() {
            let prefix = match stmt {
                ast::Statement::ExpressionStatement(e) => match e {
                    ast::Expression::Infix {
                        left,
                        operator,
                        right,
                    } => (
                        match left.as_ref() {
                            ast::Expression::Integer(i) => i,
                            _ => panic!("Invalid Right hand"),
                        },
                        operator,
                        match right.as_ref() {
                            ast::Expression::Integer(i) => i,
                            _ => panic!("Invalid Right hand"),
                        },
                    ),
                    e => panic!(format!("Invalid Infix Expression {:?}", e)),
                },
                e => panic!(format!("expect `Expression` but got {:?}", e),),
            };
            assert_eq!(*prefix.0, tests[index].0);
            assert_eq!(*prefix.1, tests[index].1);
            assert_eq!(*prefix.2, tests[index].2);
        }
    }
}
