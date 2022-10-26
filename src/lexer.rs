use core::fmt;

use crate::lox::Lox;

#[derive(PartialEq, Debug, Clone)]
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
    Identifier(String),
    Str(String),
    Number(f64),

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
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenType::*;
        match self {
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Semicolon => write!(f, ";"),
            Slash => write!(f, "/"),
            Star => write!(f, "*"),

            // One or two character tokens.
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),

            // Literals.
            Identifier(s) => write!(f, "ID({})", s),
            Str(s) => write!(f, "STRING({})", s),
            Number(n) => write!(f, "NUM({})", n),

            // Keywords.
            And => write!(f, "AND"),
            Class => write!(f, "CLASS"),
            Else => write!(f, "ELSE"),
            False => write!(f, "FALSE"),
            Fun => write!(f, "FUN"),
            For => write!(f, "FOR"),
            If => write!(f, "IF"),
            Nil => write!(f, "NIL"),
            Or => write!(f, "OR"),
            Print => write!(f, "PRINT"),
            Return => write!(f, "RETURN"),
            Super => write!(f, "SUPER"),
            This => write!(f, "THIS"),
            True => write!(f, "TRUE"),
            Var => write!(f, "VAR"),
            While => write!(f, "WHILE"),

            Eof => write!(f, "EOF"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenType::*;
        let mut literal = String::new();
        match &self.token_type {
            Identifier(s) | Str(s) => literal = format!("{s}"),
            Number(n) => literal = format!("{n}"),
            _ => {}
        }
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

#[derive(Debug, Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    lox: Lox,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut s = Self::default();
        s.source = source;
        s.line = 1;
        s
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_next('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.number();
                } else {
                    self.lox
                        .error(self.line, format!("Unexpected character {c}."));
                }
            }
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num_str = &self.source[self.start..self.current];
        if let Ok(n) = num_str.parse::<f64>() {
            self.add_token(TokenType::Number(n));
        } else {
            self.lox.error(self.line, "Expected a decimal.".to_string());
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.lox
                .error(self.line, "Unterminated string.".to_string());
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::Str(value.to_string()))
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        if let Some(c) = self.source.chars().nth(self.current) {
            return c;
        }
        '\0'
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        if let Some(c) = self.source.chars().nth(self.current + 1) {
            return c;
        }
        '\0'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch: char;
        if let Some(c) = self.source.chars().nth(self.current) {
            ch = c;
        } else {
            ch = '\0';
        }
        self.current += 1;
        ch
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), self.line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;
    #[test]
    fn test_scanner() {
        let source = "(()){}!*+-/=<> <= >= == \"foo bar baz\" = 20 \n3.14 // ".to_string();

        let mut scanner = Scanner::new(source.clone());
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 1);

        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::new(LeftParen, "(".to_string(), 1),
                Token::new(LeftParen, "(".to_string(), 1),
                Token::new(RightParen, ")".to_string(), 1),
                Token::new(RightParen, ")".to_string(), 1),
                Token::new(LeftBrace, "{".to_string(), 1),
                Token::new(RightBrace, "}".to_string(), 1),
                Token::new(Bang, "!".to_string(), 1),
                Token::new(Star, "*".to_string(), 1),
                Token::new(Plus, "+".to_string(), 1),
                Token::new(Minus, "-".to_string(), 1),
                Token::new(Slash, "/".to_string(), 1),
                Token::new(Equal, "=".to_string(), 1),
                Token::new(Less, "<".to_string(), 1),
                Token::new(Greater, ">".to_string(), 1),
                Token::new(LessEqual, "<=".to_string(), 1),
                Token::new(GreaterEqual, ">=".to_string(), 1),
                Token::new(EqualEqual, "==".to_string(), 1),
                Token::new(
                    Str("foo bar baz".to_string()),
                    "\"foo bar baz\"".to_string(),
                    1
                ),
                Token::new(Equal, "=".to_string(), 1),
                Token::new(Number(20.0), "20".to_string(), 1),
                Token::new(Number(3.14), "3.14".to_string(), 2),
                Token::new(Eof, "".to_string(), 2),
            ],
        );

        // The last lexeme seen was a three-character "// ", so the start cursor is 3 behind current.
        assert_eq!(scanner.start, source.len() - 3);
        assert_eq!(scanner.current, source.len());
    }
}
