use TokenType;

struct Token {
    type_: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl Token {
    fn to_string(&self) -> String {
        format!("{} {} {}", this.type_, this.lexeme, this.literal)
    }

    fn new(type_: TokenType, lexeme: &str, literal: &str, line: int) -> Self {
        Token {
            type_,
            lexeme,
            literal,
            line,
        }
    }
}
