use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    ILLEGAL,
    EOF,
    // identifiers
    IDENT(String),
    INT(i32),
    // operators
    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    LT,
    GT,
    BANG,
    //delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // keywards
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NEQ,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            TokenKind::ILLEGAL => "ILLEGAL",
            TokenKind::EOF => "EOF",
            TokenKind::IDENT(_) => "IDENT",
            TokenKind::INT(_) => "INT",
            TokenKind::ASSIGN => "ASSIGN",
            TokenKind::PLUS => "PLUS",
            TokenKind::MINUS => "MINUS",
            TokenKind::ASTERISK => "ASTERISK",
            TokenKind::SLASH => "SLASH",
            TokenKind::LT => "LT",
            TokenKind::GT => "GT",
            TokenKind::BANG => "BANG",
            TokenKind::COMMA => "CAMMA",
            TokenKind::SEMICOLON => "SEMICOLON",
            TokenKind::LPAREN => "LPAREN",
            TokenKind::RPAREN => "RPAREN",
            TokenKind::LBRACE => "LBRACE",
            TokenKind::RBRACE => "RBRACE",
            TokenKind::FUNCTION => "FUNCTION",
            TokenKind::LET => "LET",
            TokenKind::TRUE => "TRUE",
            TokenKind::FALSE => "FALSE",
            TokenKind::IF => "IF",
            TokenKind::ELSE => "ELSE",
            TokenKind::RETURN => "RETURN",
            TokenKind::EQ => "EQ",
            TokenKind::NEQ => "NEQ",
        };
        write!(f, "{}", value)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.literal)
    }
}
