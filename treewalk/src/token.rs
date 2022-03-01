use crate::token_type::TokenType;
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.type_, self.lexeme, self.literal)
    }

    pub fn new(type_: TokenType, lexeme: &str, literal: &str, line: usize) -> Self {
        Token {
            type_,
            lexeme: lexeme.to_string(),
            literal: literal.to_string(),
            line,
        }
    }
}
