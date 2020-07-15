use crate::ast;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError {}
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
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        None
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { statements: vec![] };
        while self.current_token.kind != TokenKind::EOF {
            let statement = self.parse_statement();
            if let Some(s) = statement {
                program.statements.push(s);
            }
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

        for s in &program.statements {
            assert_eq!(s.token_literal(), "let");
        }
    }
}
