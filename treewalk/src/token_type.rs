#[derive(Debug, Clone)]
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
    Identifier,
    String,
    Number,

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
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: i32,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"".to_string())
        )
    }
}
