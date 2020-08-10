use crate::token::*;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
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

    fn read_number(&mut self) -> Token {
        let mut ident = String::new();
        let mut is_float = false;
        while Self::is_digit(self.current) {
            ident.push(self.current);
            // 読み込みは一回だけ。それ以降は通さない
            if self.next == '.' {
                self.read_char();
                is_float = true;
                ident.push(self.current);
            }
            self.read_char();
        }

        if is_float {
            Token::FLOAT(ident.parse::<f32>().unwrap())
        } else {
            Token::INT(ident.parse::<i32>().unwrap())
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while Self::is_letter(self.current) {
            ident.push(self.current);
            self.read_char();
        }
        ident
    }

    fn token_from(&self, ident: &str) -> Token {
        match ident {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            "for" => Token::FOR,
            "in" => Token::IN,
            ident => Token::IDENT(ident.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current {
            '=' => match self.next {
                // ==の分岐
                '=' => {
                    // 先読みしているので一つすすめるのを忘れずに
                    self.read_char();
                    Token::EQ
                }
                _ => Token::ASSIGN,
            },
            '!' => match self.next {
                '=' => {
                    self.read_char();
                    Token::NEQ
                }
                _ => Token::BANG,
            },
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::COMMA,
            ':' => Token::COLON,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '[' => Token::LBRACKET,
            ']' => Token::RBRACKET,
            '"' => {
                self.read_char();
                let mut ident = String::new();
                while self.current != '"' {
                    ident.push(self.current);
                    self.read_char();
                }
                Token::STRING(ident)
            }
            '\u{0000}' => Token::EOF,
            c => {
                if Self::is_letter(c) {
                    let ident = self.read_identifier();
                    return self.token_from(&ident);
                } else if Self::is_digit(c) {
                    return self.read_number();
                } else {
                    return Token::ILLEGAL;
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
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::LT,
            Token::GT,
        ];
        for t in tests {
            let token = lexer.next_token();
            assert_eq!(token, t);
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
            let result = add(five, ten);
            if 
            else 
            return 
            true 
            false
            ==
            !=
            "foobar"
            "foo bar"
            []
            :
            5.5;
            for 
            in
            "#;

        let mut lexer = Lexer::new(input);
        let tests = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::IF,
            Token::ELSE,
            Token::RETURN,
            Token::TRUE,
            Token::FALSE,
            Token::EQ,
            Token::NEQ,
            Token::STRING("foobar".to_string()),
            Token::STRING("foo bar".to_string()),
            Token::LBRACKET,
            Token::RBRACKET,
            Token::COLON,
            Token::FLOAT(5.5),
            Token::SEMICOLON,
            Token::FOR,
            Token::IN,
            Token::EOF,
        ];
        for t in tests {
            let token = lexer.next_token();
            assert_eq!(token, t);
        }
    }
}
