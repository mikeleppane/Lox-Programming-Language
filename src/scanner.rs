use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub const fn new(source: &str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /* pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token);
        self.tokens.clone()
    } */

    fn scan_token(&mut self) {
        let c = self.advance();
        let lexeme = &self.source[self.start..self.current];
        match c {
            '(' => self.add_token(Token::new(TokenType::LeftParen, lexeme, self.line)),
            ')' => self.add_token(Token::new(TokenType::RightParen, lexeme, self.line)),
            '{' => self.add_token(Token::new(TokenType::LeftBrace, lexeme, self.line)),
            '}' => self.add_token(Token::new(TokenType::RightBrace, lexeme, self.line)),
            ',' => self.add_token(Token::new(TokenType::Comma, lexeme, self.line)),
            '.' => self.add_token(Token::new(TokenType::Dot, lexeme, self.line)),
            '-' => self.add_token(Token::new(TokenType::Minus, lexeme, self.line)),
            '+' => self.add_token(Token::new(TokenType::Plus, lexeme, self.line)),
            ';' => self.add_token(Token::new(TokenType::Semicolon, lexeme, self.line)),
            '*' => self.add_token(Token::new(TokenType::Star, lexeme, self.line)),
            '!' => {
                let token = if self.match_char('=') {
                    let lexeme = &self.source[self.start..self.current];
                    Token::new(TokenType::BangEqual, lexeme, self.line)
                } else {
                    Token::new(TokenType::Bang, lexeme, self.line)
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.match_char('=') {
                    let lexeme = &self.source[self.start..self.current];
                    Token::new(TokenType::EqualEqual, lexeme, self.line)
                } else {
                    Token::new(TokenType::Equal, lexeme, self.line)
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_char('=') {
                    let lexeme = &self.source[self.start..self.current];
                    Token::new(TokenType::LessEqual, lexeme, self.line)
                } else {
                    Token::new(TokenType::Less, lexeme, self.line)
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_char('=') {
                    let lexeme = &self.source[self.start..self.current];
                    Token::new(TokenType::GreaterEqual, lexeme, self.line)
                } else {
                    Token::new(TokenType::Greater, lexeme, self.line)
                };
                self.add_token(token);
            }
            '/' => {
                self.add_token(Token::new(TokenType::Slash, lexeme, self.line));
            }
            '"' => self.string(),
            _ => (), /* _ => {
                         if c.is_digit(10) {
                             self.number();
                         } else if c.is_alphabetic() {
                             self.identifier()
                                 .unwrap_or_else(|| self.add_token());
                         } else {
                             eprintln!("Unexpected character: {}", c);
                         }
                     } */
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth(self.current - 1)
            .expect("Unexpected end of source")
    }

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    const fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /* fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let number = self.source[self.start..self.current].parse().unwrap();
        self.add_token(Token::Number(number));
    } */

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    /* fn identifier(&mut self) -> Option<()> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        match text.as_str() {
            "and" => self.add_token(Token::And),
            "class" => self.add_token(Token::Class),
            "else" => self.add_token(Token::Else),
            "false" => self.add_token(Token::False),
            "for" => self.add_token(Token::For),
            "fun" => self.add_token(Token::Fun),
            "if" => self.add_token(Token::If),
            "nil" => self.add_token(Token::Nil),
            "or" => self.add_token(Token::Or),
            "print" => self.add_token(Token::Print),
            "return" => self.add_token(Token::Return),
            "super" => self.add_token(Token::Super),
            "this" => self.add_token(Token::This),
            "true" => self.add_token(Token::True),
            "var" => self.add_token(Token::Var),
            "while" => self.add_token(Token::While),
            _ => self.add_token(Token::Identifier(text)),
        }
        Some(())
    } */

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            eprintln!("Unterminated string at line {}", self.line);
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(Token::new(TokenType::String(value), value, self.line));
    }

    #[allow(dead_code)]
    fn error(&self, message: &str) {
        eprintln!("[line {}] Error: {}", self.line, message);
    }

    #[allow(dead_code)]
    fn error_at(&self, message: &str, start: usize, current: usize) {
        eprintln!(
            "[line {}] Error: {} at {}",
            self.line,
            message,
            &self.source[start..current]
        );
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }

                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => return,
            }
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if self.is_at_end() {
            return None;
        }
        self.start = self.current;
        self.scan_token();
        if let Some(token) = self.tokens.pop() {
            return Some(token);
        }
        None
    }
}
