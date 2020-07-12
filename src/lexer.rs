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

    fn skip_whitespace(&mut self) {
        while self.current == ' '
            || self.current == '\t'
            || self.current == '\n'
            || self.current == '\r'
        {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.current = self.next;
        // EOFになるとorに進むのでNULLのunicodeを返す
        self.next = self.input.next().unwrap_or('\u{0000}');
    }

    fn is_letter(c: char) -> bool {
        'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
    }

    fn is_digit(c: char) -> bool {
        '0' <= c && c <= '9'
    }

    fn read_number(&mut self) -> String {
        let mut ident = String::new();
        while Self::is_digit(self.current) {
            ident.push(self.current);
            self.read_char();
        }
        ident
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while Self::is_letter(self.current) {
            ident.push(self.current);
            self.read_char();
        }
        ident
    }

    fn token_kind_from(&self, literal: &str) -> TokenKind {
        match literal {
            "let" => TokenKind::LET,
            "fn" => TokenKind::FUNCTION,
            ident => TokenKind::IDENT(ident.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current {
            '=' => Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            '+' => Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string(),
            },
            '-' => Token {
                kind: TokenKind::MINUS,
                literal: "-".to_string(),
            },
            '*' => Token {
                kind: TokenKind::ASTERISK,
                literal: "*".to_string(),
            },
            '/' => Token {
                kind: TokenKind::SLASH,
                literal: "/".to_string(),
            },
            '!' => Token {
                kind: TokenKind::BANG,
                literal: "!".to_string(),
            },
            '<' => Token {
                kind: TokenKind::LT,
                literal: "<".to_string(),
            },
            '>' => Token {
                kind: TokenKind::GT,
                literal: ">".to_string(),
            },
            ',' => Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            ';' => Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            '(' => Token {
                kind: TokenKind::LPAREN,
                literal: "(".to_string(),
            },
            ')' => Token {
                kind: TokenKind::RPAREN,
                literal: ")".to_string(),
            },
            '{' => Token {
                kind: TokenKind::LBRACE,
                literal: "{".to_string(),
            },
            '}' => Token {
                kind: TokenKind::RBRACE,
                literal: "}".to_string(),
            },
            '\u{0000}' => Token {
                kind: TokenKind::EOF,
                literal: "".to_string(),
            },
            c => {
                if Self::is_letter(c) {
                    let ident = self.read_identifier();
                    return Token {
                        kind: self.token_kind_from(&ident),
                        literal: ident,
                    };
                } else if Self::is_digit(c) {
                    let ident = self.read_number();
                    return Token {
                        kind: TokenKind::INT(ident.parse().unwrap()),
                        literal: ident,
                    };
                } else {
                    return Token {
                        kind: TokenKind::ILLEGAL,
                        literal: "".to_string(),
                    };
                }
            }
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexing() {
        let input = "=+(){},;!-/*<>";
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
            Token {
                kind: TokenKind::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::BANG,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::MINUS,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::SLASH,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::ASTERISK,
                literal: "*".to_string(),
            },
            Token {
                kind: TokenKind::LT,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::GT,
                literal: ">".to_string(),
            },
        ];
        for t in tests {
            let token = lexer.next_token();
            assert_eq!(token.kind, t.kind);
            assert_eq!(token.literal, t.literal);
        }
    }

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x,y) {
                x + y;
            };
            let result = add(five, ten);"#;
        let mut lexer = Lexer::new(input);
        let tests = vec![
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                // とりあえずダミーを詰めとく。最終的にkindだけでよくなるかも
                kind: TokenKind::IDENT("five".to_string()),
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::INT(5),
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("ten".to_string()),
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::INT(10),
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("add".to_string()),
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("y".to_string()),
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("x".to_string()),
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("y".to_string()),
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("result".to_string()),
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("add".to_string()),
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("five".to_string()),
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::IDENT("ten".to_string()),
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::EOF,
                literal: "".to_string(),
            },
        ];
        for t in tests {
            let token = lexer.next_token();
            assert_eq!(token.kind, t.kind);
            assert_eq!(token.literal, t.literal);
        }
    }
}
