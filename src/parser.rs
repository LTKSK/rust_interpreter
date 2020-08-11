use crate::ast;
use crate::error::Error;
use crate::error::Error::ParseError;
use crate::lexer::Lexer;
use crate::token::Token;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // ==
    Lessgreater, // > <
    Sum,         // + -
    Product,     // * /
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
}

impl Precedence {
    fn from_token(kind: &Token) -> Self {
        match kind {
            Token::EQ | Token::NEQ => Self::Equals,
            Token::LT | Token::GT => Self::Lessgreater,
            Token::PLUS | Token::MINUS => Self::Sum,
            Token::SLASH | Token::ASTERISK => Self::Product,
            Token::LPAREN => Self::Call,
            Token::LBRACKET => Self::Index,
            _ => Self::Lowest,
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lexer,
            current_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
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

    pub fn parse_let_statement(&mut self) -> Result<ast::Statement, Error> {
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
        // assignの次のtoken。右辺の始まり
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Ok(ast::Statement::Let {
            name: identifier,
            value,
        })
    }

    pub fn parse_return_statement(&mut self) -> Result<ast::Statement, Error> {
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Ok(ast::Statement::Return(return_value))
    }

    fn parse_call_expression(
        &mut self,
        function: ast::Expression,
    ) -> Result<ast::Expression, Error> {
        Ok(ast::Expression::Call {
            function: Box::new(function),
            arguments: self.parse_expressions(Token::RPAREN)?,
        })
    }

    fn parse_index_expression(&mut self, left: ast::Expression) -> Result<ast::Expression, Error> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RBRACKET) {
            return Err(ParseError {
                msg: format!("Expect ], but got {:?}", self.peek_token),
            });
        }
        Ok(ast::Expression::Index {
            left: Box::new(left),
            index: Box::new(index),
        })
    }

    fn parse_infix(&mut self, left: ast::Expression) -> Result<ast::Expression, Error> {
        let operator = match &self.current_token {
            Token::PLUS => ast::InfixOprator::Plus,
            Token::MINUS => ast::InfixOprator::Minus,
            Token::ASTERISK => ast::InfixOprator::Asterisk,
            Token::SLASH => ast::InfixOprator::Slash,
            Token::GT => ast::InfixOprator::Gt,
            Token::LT => ast::InfixOprator::Lt,
            Token::ASSIGN => ast::InfixOprator::Assign,
            Token::EQ => ast::InfixOprator::Equal,
            Token::NEQ => ast::InfixOprator::Nequal,
            Token::LPAREN => {
                return Ok(self.parse_call_expression(left))?;
            }
            Token::LBRACKET => {
                return Ok(self.parse_index_expression(left))?;
            }
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

    fn parse_group_expression(&mut self) -> Result<ast::Expression, Error> {
        // この時点ではparenが来てるので読みすすめる
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RPAREN) {
            return Err(ParseError {
                msg: "Parentheses are not closed. in if expression".to_string(),
            });
        }
        Ok(expression)
    }

    fn parse_block_statement(&mut self) -> Result<ast::Statement, Error> {
        self.next_token();
        let mut statements = vec![];
        while !self.current_token_is(Token::RBRACE) && !self.current_token_is(Token::EOF) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
        Ok(ast::Statement::Block(statements))
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, Error> {
        if !self.expect_peek(Token::LPAREN) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted LPAREN", self.peek_token),
            });
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RPAREN) {
            return Err(ParseError {
                msg: format!("Parentheses are not closed. in if expression"),
            });
        }
        //if文の中身
        if !self.expect_peek(Token::LBRACE) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted `{{`", self.peek_token),
            });
        }
        let consequence = self.parse_block_statement()?;

        let alternative = if self.expect_peek(Token::ELSE) {
            if !self.expect_peek(Token::LBRACE) {
                return Err(ParseError {
                    msg: format!("Unexpected token {:?}. wanted `{{`", self.peek_token),
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

    fn parse_function_params(&mut self) -> Result<Vec<ast::Expression>, Error> {
        if self.peek_token_is(Token::RPAREN) {
            self.next_token();
            return Ok(vec![]);
        }
        self.next_token();
        let mut identifiers = vec![];
        let identifier = match self.current_token.clone() {
            Token::IDENT(ident) => ast::Expression::Identifier(ident),
            t => {
                return Err(ParseError {
                    msg: format!("Expected identifier, but got {:?}", t),
                })
            }
        };
        identifiers.push(identifier);

        while self.peek_token_is(Token::COMMA) {
            // camma消費。次のtokenをcurrentにする
            self.next_token();
            self.next_token();

            let identifier = match self.current_token.clone() {
                Token::IDENT(ident) => ast::Expression::Identifier(ident),
                t => {
                    return Err(ParseError {
                        msg: format!("Expected identifier, but got {:?}", t),
                    })
                }
            };
            identifiers.push(identifier);
        }

        if !self.expect_peek(Token::RPAREN) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted `}}`", self.peek_token),
            });
        }
        Ok(identifiers)
    }

    fn parse_function_expression(&mut self) -> Result<ast::Expression, Error> {
        if !self.expect_peek(Token::LPAREN) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted `(`", self.peek_token),
            });
        }
        let parameters = self.parse_function_params()?;
        if !self.expect_peek(Token::LBRACE) {
            return Err(ParseError {
                msg: format!("Unexpected token {:?}. wanted `{{`", self.peek_token),
            });
        }
        let body = self.parse_block_statement()?;
        Ok(ast::Expression::Function {
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_expressions(&mut self, token: Token) -> Result<Vec<ast::Expression>, Error> {
        let mut expressions = vec![];
        if self.peek_token_is(token.clone()) {
            self.next_token();
            return Ok(expressions);
        }
        self.next_token();
        expressions.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_token_is(Token::COMMA) {
            // comma消費して次のtokenをみる
            self.next_token();
            self.next_token();
            expressions.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.expect_peek(token.clone()) {
            return Err(ParseError {
                msg: format!("Expect {} but got {}", token, self.peek_token),
            });
        }
        Ok(expressions)
    }

    fn parse_array_expression(&mut self) -> Result<ast::Expression, Error> {
        let expressions = self.parse_expressions(Token::RBRACKET)?;
        Ok(ast::Expression::Array(expressions))
    }

    fn parse_map_expression(&mut self) -> Result<ast::Expression, Error> {
        let mut m = BTreeMap::new();
        while !self.peek_token_is(Token::RBRACE) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;
            if !self.expect_peek(Token::COLON) {
                return Err(ParseError {
                    msg: format!("Expect `:` but got {}", self.peek_token),
                });
            }
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            m.insert(Box::new(key), Box::new(value));
            if !self.peek_token_is(Token::RBRACE) && !self.expect_peek(Token::COMMA) {
                return Err(ParseError {
                    msg: format!("Expect `}}` or `:` but got {}", self.peek_token),
                });
            }
        }

        if !self.expect_peek(Token::RBRACE) {
            return Err(ParseError {
                msg: format!("Expect `}}` but got {}", self.peek_token),
            });
        }
        Ok(ast::Expression::Map(m))
    }

    fn parse_for_expression(&mut self) -> Result<ast::Expression, Error> {
        // forの次に進む
        self.next_token();
        // identifierをparse
        let parameter = match self.parse_expression(Precedence::Lowest)? {
            ast::Expression::Identifier(i) => i,
            o => {
                return Err(Error::EvalError {
                    msg: format!("Expect identifier but got {}", o),
                })
            }
        };
        // inを読み込み(なければエラー)
        if !self.expect_peek(Token::IN) {
            return Err(Error::EvalError {
                msg: format!("Expect `in` but got {}", self.peek_token),
            });
        }
        if !self.expect_peek(Token::LBRACKET) {
            return Err(Error::EvalError {
                msg: format!("Expect `[` but got {}", self.peek_token),
            });
        }
        // arrayのparse
        let array = self.parse_array_expression()?;
        // {}内のparse
        if !self.expect_peek(Token::LBRACE) {
            return Err(Error::EvalError {
                msg: format!("Expect `{{` but got {}", self.peek_token),
            });
        }
        let statement = self.parse_block_statement()?;
        Ok(ast::Expression::For {
            parameter,
            array: Box::new(array),
            statement: Box::new(statement),
        })
    }

    fn parse_prefix(&mut self) -> Result<ast::Expression, Error> {
        match self.current_token.clone() {
            Token::IDENT(ident) => {
                if self.peek_token_is(Token::ASSIGN) {
                    self.next_token();
                    self.parse_infix(ast::Expression::Identifier(ident))
                } else {
                    Ok(ast::Expression::Identifier(ident))
                }
            }
            Token::INT(i) => Ok(ast::Expression::Integer(i)),
            Token::STRING(s) => Ok(ast::Expression::String(s)),
            Token::TRUE => Ok(ast::Expression::Bool(true)),
            Token::FALSE => Ok(ast::Expression::Bool(false)),
            Token::IF => self.parse_if_expression(),
            Token::FUNCTION => self.parse_function_expression(),
            Token::MINUS => {
                self.next_token();
                Ok(ast::Expression::Prefix {
                    operator: ast::PrefixOprator::Minus,
                    right: Box::new(self.parse_expression(Precedence::Prefix)?),
                })
            }
            Token::LPAREN => self.parse_group_expression(),
            Token::LBRACKET => self.parse_array_expression(),
            Token::LBRACE => self.parse_map_expression(),
            Token::BANG => {
                self.next_token();
                Ok(ast::Expression::Prefix {
                    operator: ast::PrefixOprator::Bang,
                    right: Box::new(self.parse_expression(Precedence::Prefix)?),
                })
            }
            Token::FOR => self.parse_for_expression(),
            t => Err(ParseError {
                msg: format!("Unexpected Prefix Expression: {}", t),
            }),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression, Error> {
        let mut expression = self.parse_prefix()?;
        while !self.peek_token_is(Token::SEMICOLON) && precedence < self.peek_precedence() {
            self.next_token();
            expression = self.parse_infix(expression)?;
        }
        Ok(expression)
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, Error> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }
        Ok(ast::Statement::Expression(expression))
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        match &self.current_token {
            Token::LET => Ok(self.parse_let_statement()?),
            Token::RETURN => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Error> {
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
        let tests = vec!["let x = 5;", "let y = 10;", "let foobar = 838383;"];
        for (index, stmt) in program.statements.iter().enumerate() {
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
        let tests = vec!["return 5;", "return 10;", "return 993322;"];
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
                    assert_eq!(format!("{}", condition.as_ref()), "(x < y)");
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
                    assert_eq!(format!("{}", condition.as_ref()), "(x < y)");
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
                    assert_eq!(format!("{}", parameters[0]), "x");
                    assert_eq!(format!("{}", parameters[1]), "y");
                    assert_eq!(format!("{}", body), "(x + y)");
                }
                e => panic!(format!("Invalid Function Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_call_expression() {
        let input = r#"
          add(1, 2*3, 4+5);
        "#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        // panicしなければOK
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::Call {
                    function,
                    arguments,
                } => {
                    assert_eq!(format!("{}", arguments[0]), "1");
                    assert_eq!(format!("{}", arguments[1]), "(2 * 3)");
                    assert_eq!(format!("{}", arguments[2]), "(4 + 5)");
                    assert_eq!(format!("{}", function), "add");
                    assert_eq!(format!("{}", e), "add(1, (2 * 3), (4 + 5))");
                }
                e => panic!(format!("Invalid Function Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_parse_string() {
        let input = r#""hello world""#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::String(s) => {
                    assert_eq!(format!("{}", s), "hello world");
                }
                e => panic!(format!("Invalid String Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_parse_array() {
        let input = r#"[1, 2]"#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::Array(array) => {
                    for (expr, expect) in array.iter().zip(vec![1, 2]) {
                        if let ast::Expression::Integer(i) = expr {
                            assert_eq!(*i, expect);
                        } else {
                            panic!("unexpected expression {:?}", expr)
                        }
                    }
                }
                e => panic!(format!("Invalid String Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_parse_index() {
        let input = "hoge[1+2];";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        let stmt = &program.statements[0];
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::Index { .. } => assert_eq!(format!("{}", e), "hoge[(1 + 2)]"),
                e => panic!(format!("Invalid String Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_parse_map() {
        let input = r#"{"one":1, "two": 2}"#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap_or_else(|e| panic!("{:?}", e));
        let stmt = &program.statements[0];
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::Map(_) => assert_eq!(format!("{}", e), "{ one: 1, two: 2 }"),
                e => panic!(format!("Invalid String Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_parse_for() {
        let input = r#" for a in [1,2,3] { a; }"#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap_or_else(|e| panic!("{:?}", e));
        let stmt = &program.statements[0];
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::For {
                    parameter,
                    array,
                    statement,
                } => {
                    assert_eq!(format!("{}", parameter), "a");
                    assert_eq!(format!("{}", array), "[1, 2, 3]");
                    assert_eq!(format!("{}", statement), "a");
                }
                e => panic!(format!("Invalid For Expression {:?}", e)),
            },
            e => panic!(format!("expect `Expression` but got {:?}", e),),
        };
    }

    #[test]
    fn test_parse_assign() {
        let input = r#"let a = 10; a = 11;"#;
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap_or_else(|e| panic!("{:?}", e));
        let stmt = &program.statements[1];
        match stmt {
            ast::Statement::Expression(e) => match e {
                ast::Expression::Infix { .. } => {
                    assert_eq!(format!("{}", e), "(a = 11)");
                }
                e => panic!(format!("Invalid Assign Expression {:?}", e)),
            },
            e => panic!(format!(
                "expect `Expression` but got {:?}, statement => {}",
                e, stmt
            ),),
        };
    }
}
