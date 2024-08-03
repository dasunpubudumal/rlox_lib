use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub struct Literal<T> {
    pub kind: T,
}

impl<T> Display for Literal<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    String(String),
    Int(i32),
    Float(f64),
}

/// Different types of tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Eof,
    Start,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
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
}

/// Token type for lexical analysis
#[derive(Debug)]
pub struct Token<T: Debug> {
    // Type of the token. I have used `kind` here as `type` is a reserved word in Rust
    pub kind: TokenType,
    // For example, in the statement int x = 10;, there would be four lexemes: `"int"`, `"x"`, `"="`, and `"10"`.
    pub lexeme: String,
    // For many tokens, this would be None, but for literals this needs to be set as its own type (e.g. `String` for strings)
    pub literal: Option<Literal<T>>,
    // Current line number
    pub line: usize,
}

#[derive(Debug)]
pub struct TokenBuilder<T> {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal<T>>,
    pub line: usize,
}

impl<T: Debug> TokenBuilder<T> {
    pub fn new() -> TokenBuilder<T> {
        TokenBuilder {
            kind: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line: 0,
        }
    }

    pub fn kind(mut self, kind: TokenType) -> Self {
        self.kind = kind;
        self
    }

    pub fn lexeme(mut self, lexeme: String) -> Self {
        self.lexeme = lexeme;
        self
    }

    pub fn literal(mut self, literal: Option<Literal<T>>) -> Self {
        self.literal = literal;
        self
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    pub fn build(self) -> Token<T> {
        Token {
            kind: self.kind,
            lexeme: self.lexeme,
            literal: self.literal,
            line: self.line,
        }
    }
}

// It's possible to implement system traits; in this case it's converting a Token instance into a String
impl<T: Debug> From<Token<T>> for String {
    fn from(value: Token<T>) -> Self {
        format!("{:?} {:?} {:?}", value.kind, value.lexeme, value.literal)
    }
}
