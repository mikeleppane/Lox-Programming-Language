use core::fmt;

use crate::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    Bool(bool),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Str(s) => write!(f, "{s}"),
            Self::Nil => write!(f, "nil"),
            Self::Bool(b) => write!(f, "{b}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    #[must_use]
    pub const fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
    ) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    #[must_use]
    pub fn get_type(&self) -> TokenType {
        self.ttype.clone()
    }

    #[must_use]
    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    #[must_use]
    pub fn get_literal(&self) -> Option<Object> {
        self.literal.clone()
    }

    #[must_use]
    pub const fn get_line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn eof(line: usize) -> Self {
        Self {
            ttype: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.ttype,
            self.lexeme,
            self.literal
                .as_ref()
                .map_or_else(String::new, |lit| format!("{lit:?}"))
        )
    }
}
