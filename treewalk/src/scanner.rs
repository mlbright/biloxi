use TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source,
            line: 1,
        }
    }

    fn scan_tokens() -> Vec<Token> {
        while !is_at_end() {
            start = current;
            scan_token();
        }
        tokens.add(Token::new(TokenType::EOF, "", "", line));
        return tokens;
    }

    fn is_at_end(&self) -> bool {
        return this.current >= source.len();
    }
}
