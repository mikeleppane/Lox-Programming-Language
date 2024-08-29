#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    r#type: TokenType<'a>,
    lexeme: &'a str,
    line: usize,
}

impl<'a> Token<'a> {
    pub const fn new(token_type: TokenType<'a>, lexeme: &'a str, line: usize) -> Self {
        Token {
            r#type: token_type,
            lexeme,
            line,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(&'a str),
    String(&'a str),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // End of file.
    Eof,
}
