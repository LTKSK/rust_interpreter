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

    fn current_token_is(&self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            return true;
        }
        return false;
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        if !self.expect_peek(TokenKind::IDENT) {
            panic!("identのpeek失敗");
            //return None;
        }
        let identifier = ast::Identifier {
            token: self.current_token.clone(),
        };
        if !self.expect_peek(TokenKind::ASSIGN) {
            panic!("=のpeek失敗");
            //return None;
        }
        // TODO: implelemnt expression
        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }
        let stmt = ast::Statement::LetStatement {
            name: identifier,
            // TODO 上でexpressionのparseが実装されたらNoneを置き換える
            value: ast::Expression::None,
        };
        Some(stmt)
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match &self.current_token.kind {
            TokenKind::LET => self.parse_let_statement(),
            _ => None,
        }
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

        //for s in &program.statements {
        //    assert_eq!(s.token_literal(), "let");
        //}
    }
}
