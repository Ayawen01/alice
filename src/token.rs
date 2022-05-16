#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: Option<String>,
    pub literal: Option<Literal>,
    pub line: u32
}

#[derive(Debug)]
pub enum Literal {
    Id(String),
    String(String),
    Array(Vec<Literal>),
    I64(i64),
    F64(f64),
    True,
    False,
    Nil
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace, LeftSquare, RightSquare,
    Comma, Dot, Minus, Plus, Slash, Star, Semicolon,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Arrowhead,

    // Literals.
    Identifier, String, I64, F64, Array, True, False, Nil,

    // Keywords.
    And, Or, For, In, If, Else, Fn, Println, Return, Let,

    Eof
}