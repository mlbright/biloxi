use crate::token::Token;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 1,
            current: 1,
            line: 1,
        }
    }

    fn scan_tokens() -> Vec<Token> {
        while !is_at_end() {
            start = current;
            scan_token();
        }
        tokens.add(Token::new(TokenType::Eof, "", "", line));
        return tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}
