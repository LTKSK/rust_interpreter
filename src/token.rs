use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    // identifiers
    IDENT(String),
    INT(i32),
    STRING(String),
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
    LBRACKET,
    RBRACKET,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Token::ILLEGAL => "ILLEGAL".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::IDENT(s) => s.to_string(),
            Token::INT(i) => format!("{}", i),
            Token::STRING(s) => format!("{}", s),
            Token::ASSIGN => "ASSIGN".to_string(),
            Token::PLUS => "PLUS".to_string(),
            Token::MINUS => "MINUS".to_string(),
            Token::ASTERISK => "ASTERISK".to_string(),
            Token::SLASH => "SLASH".to_string(),
            Token::LT => "LT".to_string(),
            Token::GT => "GT".to_string(),
            Token::BANG => "BANG".to_string(),
            Token::COMMA => "CAMMA".to_string(),
            Token::SEMICOLON => "SEMICOLON".to_string(),
            Token::LPAREN => "LPAREN".to_string(),
            Token::RPAREN => "RPAREN".to_string(),
            Token::LBRACE => "LBRACE".to_string(),
            Token::RBRACE => "RBRACE".to_string(),
            Token::LBRACKET => "LBRACKET".to_string(),
            Token::RBRACKET => "RBRACKET".to_string(),
            Token::FUNCTION => "FUNCTION".to_string(),
            Token::LET => "LET".to_string(),
            Token::TRUE => "TRUE".to_string(),
            Token::FALSE => "FALSE".to_string(),
            Token::IF => "IF".to_string(),
            Token::ELSE => "ELSE".to_string(),
            Token::RETURN => "RETURN".to_string(),
            Token::EQ => "EQ".to_string(),
            Token::NEQ => "NEQ".to_string(),
        };
        write!(f, "{}", value)
    }
}
