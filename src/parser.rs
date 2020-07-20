use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;
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
    fn from_token(kind: &Token) -> Self {
        match kind {
            Token::EQ | Token::NEQ => Self::Equals,
            Token::LT | Token::GT => Self::Lessgreater,
            Token::PLUS | Token::MINUS => Self::Sum,
            Token::SLASH | Token::ASTERISK => Self::Product,
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
            current_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
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

    fn current_token_is(&self, kind: Token) -> bool {
        self.current_token == kind
    }

    fn peek_token_is(&self, kind: Token) -> bool {
        self.peek_token == kind
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token(&self.peek_token)
    }

    fn expect_peek(&mut self, kind: Token) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            return true;
        }
        return false;
    }

    pub fn parse_let_statement(&mut self) -> Result<ast::Statement, ParseError> {
        self.next_token();
        let identifier = match self.current_token.clone() {
            Token::IDENT(ident) => ident,
            _ => {
                return Err(ParseError {
                    msg: format!("expect `IDENT` but got `{}`", self.peek_token),
                })
            }
        };
        if !self.expect_peek(Token::ASSIGN) {
            return Err(ParseError {
                msg: format!("expect `IDENT` but got `{}`", self.peek_token),
            });
        }
        // TODO: implelemnt expression
        while !self.current_token_is(Token::SEMICOLON) {
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
        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Ok(stmt)
    }

    fn parse_infix(&mut self, left: ast::Expression) -> Result<ast::Expression, ParseError> {
        let operator = match &self.current_token {
            Token::PLUS => ast::InfixOprator::Plus,
            Token::MINUS => ast::InfixOprator::Minus,
            Token::ASTERISK => ast::InfixOprator::Asterisk,
            Token::SLASH => ast::InfixOprator::Slash,
            Token::GT => ast::InfixOprator::Gt,
            Token::LT => ast::InfixOprator::Lt,
            Token::EQ => ast::InfixOprator::Equal,
            Token::NEQ => ast::InfixOprator::Nequal,
            _ => return Ok(left),
        };

        let precedence = Precedence::from_token(&self.current_token);
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(ast::Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_group_expression(&mut self) -> Result<ast::Expression, ParseError> {
        // この時点ではparenが来てるので読みすすめる
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RPAREN) {
            return Err(ParseError {
                msg: "Parentheses are not closed.".to_string(),
            });
        }
        Ok(expression)
    }

    fn parse_block_statement(&mut self) -> Result<ast::Statement, ParseError> {
        self.next_token();
        let mut statements = vec![];
        while !self.current_token_is(Token::RBRACE) && !self.current_token_is(Token::EOF) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
        Ok(ast::Statement::Block(statements))
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, ParseError> {
        if !self.expect_peek(Token::LPAREN) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted LPAREN", self.peek_token),
            });
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RPAREN) {
            return Err(ParseError {
                msg: format!("Parentheses are not closed."),
            });
        }
        //if文の中身
        if !self.expect_peek(Token::LBRACE) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted LBRACE", self.peek_token),
            });
        }
        let consequence = self.parse_block_statement()?;

        let alternative = if self.expect_peek(Token::ELSE) {
            if !self.expect_peek(Token::LBRACE) {
                return Err(ParseError {
                    msg: format!("Unexpected token {:?}. wanted LBRACE", self.peek_token),
                });
            }
            let stmt = self.parse_block_statement()?;
            Some(Box::new(stmt))
        } else {
            None
        };
        Ok(ast::Expression::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        })
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression, ParseError> {
        match self.current_token.clone() {
            Token::IDENT(ident) => Ok(ast::Expression::Identifier(ident)),
            Token::INT(i) => Ok(ast::Expression::Integer(i)),
            Token::TRUE => Ok(ast::Expression::Bool(true)),
            Token::FALSE => Ok(ast::Expression::Bool(false)),
            Token::IF => self.parse_if_expression(),
            Token::MINUS => {
                self.next_token();
                Ok(ast::Expression::Prefix {
                    operator: ast::PrefixOprator::Minus,
                    right: Box::new(self.parse_expression(Precedence::Prefix)?),
                })
            }
            Token::LPAREN => self.parse_group_expression(),
            Token::BANG => {
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

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression, ParseError> {
        let mut expression = self.parse_prefix()?;
        while !self.peek_token_is(Token::SEMICOLON) && precedence < self.peek_precedence() {
            self.next_token();
            expression = self.parse_infix(expression)?;
        }
        Ok(expression)
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Ok(ast::Statement::Expression(expression))
    }

    pub fn parse_statement(&mut self) -> Result<ast::Statement, ParseError> {
        match &self.current_token {
            Token::LET => Ok(self.parse_let_statement()?),
            Token::RETURN => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { statements: vec![] };
        while !self.current_token_is(Token::EOF) {
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
        let tests = vec!["return dummy;", "return dummy;", "return dummy;"];
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
            ast::Statement::Expression(e) => match e {
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
            ast::Statement::Expression(e) => match e {
                ast::Expression::Integer(i) => i,
                _ => &999,
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
        assert_eq!(ident, &5);
    }

    #[test]
    fn test_bool_expression() {
        let input = r#"
            true;
            false;
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 2);

        let tests = [true, false];
        for (index, stmt) in program.statements.iter().enumerate() {
            let exp = match stmt {
                ast::Statement::Expression(e) => match e {
                    ast::Expression::Bool(b) => b,
                    _ => panic!(format!("expect `Bool` but got {:?}", e),),
                },
                e => panic!(format!("expect `Expression` but got {:?}", e),),
            };
            assert_eq!(exp, &tests[index]);
        }
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
                ast::Statement::Expression(e) => match e {
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
            let infix = match stmt {
                ast::Statement::Expression(e) => match e {
                    ast::Expression::Infix {
                        left,
                        operator,
                        right,
                    } => (
                        match left.as_ref() {
                            ast::Expression::Integer(i) => i,
                            _ => panic!("Invalid left hand"),
                        },
                        operator,
                        match right.as_ref() {
                            ast::Expression::Integer(i) => i,
                            _ => panic!("Invalid right hand"),
                        },
                    ),
                    e => panic!(format!("Invalid Infix Expression {:?}", e)),
                },
                e => panic!(format!("expect `Expression` but got {:?}", e),),
            };
            assert_eq!(*infix.0, tests[index].0);
            assert_eq!(*infix.1, tests[index].1);
            assert_eq!(*infix.2, tests[index].2);
        }
    }

    #[test]
    fn test_infix_expression_with_bool() {
        let input = r#"
            true == true;
            true != false;
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 2);

        let tests = [
            (true, ast::InfixOprator::Equal, true),
            (true, ast::InfixOprator::Nequal, false),
        ];

        for (index, stmt) in program.statements.iter().enumerate() {
            let infix = match stmt {
                ast::Statement::Expression(e) => match e {
                    ast::Expression::Infix {
                        left,
                        operator,
                        right,
                    } => (
                        match left.as_ref() {
                            ast::Expression::Bool(b) => b,
                            _ => panic!("Invalid left hand"),
                        },
                        operator,
                        match right.as_ref() {
                            ast::Expression::Bool(b) => b,
                            _ => panic!("Invalid right hand"),
                        },
                    ),
                    e => panic!(format!("Invalid Infix Expression {:?}", e)),
                },
                e => panic!(format!("expect `Expression` but got {:?}", e),),
            };
            assert_eq!(*infix.0, tests[index].0);
            assert_eq!(*infix.1, tests[index].1);
            assert_eq!(*infix.2, tests[index].2);
        }
    }

    #[test]
    fn test_if_expression() {
        let input = r#"
          if (x < y){ x };
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        // panicしなければOK
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::If {
                    condition,
                    consequence,
                    alternative,
                } => {
                    assert_eq!(format!("{}", condition.as_ref()), "x < y");
                    assert_eq!(format!("{}", consequence.as_ref()), "x");
                    if let Some(a) = alternative {
                        assert_eq!(format!("{}", a.as_ref()), "y");
                    };
                }
                e => panic!(format!("Invalid Infix Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_if_else_expression() {
        let input = r#"
          if (x < y){ x } else { y };
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        // panicしなければOK
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::If {
                    condition,
                    consequence,
                    alternative,
                } => {
                    assert_eq!(format!("{}", condition.as_ref()), "x < y");
                    assert_eq!(format!("{}", consequence.as_ref()), "x");
                    if let Some(a) = alternative {
                        assert_eq!(format!("{}", a.as_ref()), "y");
                    };
                }
                e => panic!(format!("Invalid Infix Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    fn test_function_expression() {
        let input = r#"
          fn (x, y){ x + y; }
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        // panicしなければOK
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::Function { parameters, body } => {
                    //assert_eq!(format!("{}", parameters), "x < y");
                }
                e => panic!(format!("Invalid Infix Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }
}
