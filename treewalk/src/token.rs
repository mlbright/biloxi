use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn to_string(&self) -> String {
        format!(
            "{:?} {} {}",
            self.type_,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"default".to_string())
        )
    }

    pub fn new(type_: TokenType, lexeme: &str, literal: Option<&str>, line: usize) -> Self {
        Token {
            type_,
            lexeme: lexeme.to_string(),
            literal: Some(literal.unwrap_or("default").to_string()),
            line,
        }
    }
}
