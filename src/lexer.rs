use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    ILLEGAL,
    EOF,
    // identifiers
    IDENT(String),
    INT(i32),
    // operators
    ASSIGN,
    PLUS,
    //delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    // keywards
    FUNCTION,
    LET,
    NONE,
}

#[derive(Clone, Debug)]
struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(Clone, Debug)]
struct Lexer<'a> {
    input: Chars<'a>,
    current: char,
    next: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: '\u{0000}',
            next: '\u{0000}',
        };
        // currentが1文字目になるよう最初に2回読み込む
        lexer.read_char();
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.current {
            '=' => Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            '+' => Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string(),
            },
            '\u{0000}' => Token {
                kind: TokenKind::NONE,
                literal: "".to_string(),
            },
            _ => unreachable!(),
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.current = self.next;
        self.next = self.input.next().unwrap_or('\u{0000}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        //let input = "=+(){},;";
        let input = "=+";
        let mut lexer = Lexer::new(input);
        let tests = vec![
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string(),
            },
        ];
        for t in tests {
            let token = lexer.next_token();
            assert_eq!(token.kind, t.kind);
            assert_eq!(token.literal, t.literal);
        }
    }
}
