use crate::token::Token;
use crate::token_type::TokenType;
struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "", "", self.line));

        return self.tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            _ => panic!("Unmatched something or other"),
        }
    }

    fn advance(&self) -> char {
        self.source[self.current+1]
    }

    fn add_token(&self, type_: TokenType, literal: Option<String>) {
        let text = self.source[self.start:self.current];
        match literal {
            Some(s) => self.tokens.add(Token::new(type_, &text, literal, self.line)),
            None => self.tokens.add(Token::new(type_, None)),
        }
    }
}
