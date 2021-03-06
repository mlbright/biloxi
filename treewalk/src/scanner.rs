use crate::token::Token;
use crate::token_type::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));

        return self.tokens.clone();
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.match_char('=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Bang, None),
            },
            '=' => match self.match_char('=') {
                true => self.add_token(TokenType::EqualEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '<' => match self.match_char('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.match_char('=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' => (),
            '\t' => (),
            '\r' => (),
            '\n' => self.line += 1,
            '"' => self.stringify(),
            'o' => {
                if self.match_char('r') {
                    self.add_token(TokenType::Or, None);
                }
            }
            d if d.is_digit(10) => self.number(),
            c if c.is_alphanumeric() => self.identifier(),
            _ => {
                crate::error(self.line, "Unexpected character.");
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match text {
            "and" => self.add_token(TokenType::And, None),
            "class" => self.add_token(TokenType::Class, None),
            "else" => self.add_token(TokenType::Else, None),
            "false" => self.add_token(TokenType::False, None),
            "for" => self.add_token(TokenType::For, None),
            "fun" => self.add_token(TokenType::Fun, None),
            "if" => self.add_token(TokenType::If, None),
            "nil" => self.add_token(TokenType::Nil, None),
            "or" => self.add_token(TokenType::Or, None),
            "print" => self.add_token(TokenType::Print, None),
            "return" => self.add_token(TokenType::Return, None),
            "super" => self.add_token(TokenType::Super, None),
            "this" => self.add_token(TokenType::This, None),
            "true" => self.add_token(TokenType::True, None),
            "var" => self.add_token(TokenType::Var, None),
            "while" => self.add_token(TokenType::While, None),
            _ => self.add_token(TokenType::Identifier, None),
        }
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let number = &self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number, Some(Literal::Number(*number)));
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap_or('x')
        }
    }

    fn stringify(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::error(self.line, "Unterminated string.");
        }

        self.advance(); // The closing ".

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::String(value.to_string())));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current).unwrap_or('x')
    }

    fn add_token(&mut self, type_: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        match literal {
            Some(s) => self
                .tokens
                .push(Token::new(type_, text, Some(s), self.line)),
            None => self.tokens.push(Token::new(type_, text, None, self.line)),
        }
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap_or('x') != c {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap_or('x')
        }
    }
}
