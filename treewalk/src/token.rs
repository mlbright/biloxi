use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<crate::scanner::Literal>,
    line: usize,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "{:?} {} {}",
            self.type_,
            self.lexeme,
            self.literal.as_ref().unwrap(),
        )
    }

    pub fn new(
        type_: TokenType,
        lexeme: &str,
        literal: Option<crate::scanner::Literal>,
        line: usize,
    ) -> Self {
        Token {
            type_,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}
