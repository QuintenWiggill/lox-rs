use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
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
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    NumberLiteral,
    StringLiteral,
    Identifier,
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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
    EoF,

    UnexpectedCharacterError,
    UnterminatedStringError,
}

impl TokenType {
    pub fn error_message(t: TokenType) -> Option<&'static str> {
        match t {
            Self::UnexpectedCharacterError => Some("Unexpected character."),
            Self::UnterminatedStringError => Some("Unterminated string."),
            _ => None,
        }
    }
    pub fn report(line: u32, _where: String, message: &str) {
        println!("[line {line}] Error {_where} {message}");
    }

    pub fn error(t: TokenType, line: u32) {
        let error_message = TokenType::error_message(t); 
        if let Some(error_message) = error_message {
            TokenType::report(line, "".to_string(), error_message);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

fn is_digit(c: Option<char>) -> bool {
    if let Some(c) = c {
        return ('0'..='9').contains(&c)
    }
    false
}

fn is_ident(c: Option<char>) -> bool {
    if let Some(c) = c {
        return ('A'..='Z').contains(&c) || ('a'..='z').contains(&c) || c == '_';
    }
    false
}

fn check_keyword(word: &str, kw: &str, pos: usize, tt: TokenType) -> TokenType {
    if word[pos..] == kw[pos..] {
        tt
    } else {
        TokenType::Identifier
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    token_start: usize,
    chars: Peekable<CharIndices<'a>>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.char_indices().peekable();
        Self {
            source,
            token_start: chars.peek().map(|(index, _c)| *index).unwrap_or(0),
            chars,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|(_index, c)| c)
    }

    fn maybe_match(&mut self, expected: char) -> bool {
        match self.chars.peek() {
            None => false,
            Some((_index, c)) => {
                if *c != expected {
                    return false;
                }
                let _ = self.advance();
                true
            }
        }
    }

    fn maybe_match_str(&mut self, expected: &str) -> bool {
        let strlen = expected.len();
        let byte: usize = match self.chars.peek() {
            None => return false,
            Some((index, _c)) => *index,
        };
        let end_offset = byte + strlen;
        if end_offset > self.source.len() {
            return false;
        }
        if expected == &self.source[byte..end_offset] {
            for _ in 0..expected.chars().count() {
                self.chars.next();
            }
            return true;
        }
        false
    }

    fn current(&mut self) -> usize {
        self.chars
            .peek()
            .map(|(index, _c)| *index)
            .unwrap_or(self.source.len())
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some((_, ' ')) | Some((_, '\r')) | Some((_, '\t')) => {
                    self.advance();
                }
                Some((_, '\n')) => {
                    self.line += 1;
                    self.advance();
                }
                Some((_, '/')) => {
                    if self.maybe_match_str("//") {
                        while let Some((_, c)) = self.chars.peek() {
                            if *c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            };
        }
    }

    fn content(&mut self) -> &'a str {
        let current = self.current();
        &self.source[self.token_start..current]
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token::new(token_type, self.content().to_string(), self.line)
    }

    fn string_literal(&mut self) -> Token {
        loop {
            match self.chars.peek() {
                Some((_, '"')) => {
                    self.advance();
                    return self.make_token(TokenType::StringLiteral);
                }
                Some((_, c)) => {
                    if *c == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                None => return self.make_token(TokenType::UnterminatedStringError),
            }
        }
    }

    fn consume_integers(&mut self) {
        while match self.chars.peek() {
            Some((_, c)) => is_digit(Some(*c)),
            None => false,
        } {
            self.advance();
        }
    }

    fn number_literal(&mut self) -> Token {
        self.consume_integers();
        let mut ch = self.chars.clone();
        if let Some((_, '.')) = ch.next() {
            if let Some((_, c)) = ch.next() {
                if is_digit(Some(c)) {
                    self.advance();
                    self.consume_integers();
                }
            }
        }
        self.make_token(TokenType::NumberLiteral)
    }

    fn identifier(&mut self) -> Token {
        while match self.chars.peek() {
            Some((_, c)) => is_digit(Some(*c)) || is_ident(Some(*c)),
            None => false,
        } {
            self.advance();
        }
        let t = self.identifier_type();
        self.make_token(t)
    }

    fn identifier_type(&mut self) -> TokenType {
        let word = self.content();
        if word.is_empty() {
            return TokenType::Identifier;
        }
        match &word[..1] {
            "a" => check_keyword(word, "and", 1, TokenType::And),
            "c" => check_keyword(word, "class", 1, TokenType::Class),
            "e" => check_keyword(word, "else", 1, TokenType::Else),
            "f" => {
                if word.len() < 2 {
                    return TokenType::Identifier;
                }
                match &word[1..2] {
                    "a" => check_keyword(word, "false", 2, TokenType::False),
                    "o" => check_keyword(word, "for", 2, TokenType::For),
                    "u" => check_keyword(word, "fun", 2, TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            }
            "i" => check_keyword(word, "if", 1, TokenType::If),
            "n" => check_keyword(word, "nil", 1, TokenType::Nil),
            "o" => check_keyword(word, "or", 1, TokenType::Or),
            "p" => check_keyword(word, "print", 1, TokenType::Print),
            "r" => check_keyword(word, "return", 1, TokenType::Return),
            "s" => check_keyword(word, "super", 1, TokenType::Super),
            "t" => {
                if word.len() < 2 {
                    return TokenType::Identifier;
                }
                match &word[1..2] {
                    "h" => check_keyword(word, "this", 2, TokenType::This),
                    "r" => check_keyword(word, "true", 2, TokenType::True),
                    _ => TokenType::Identifier,
                }
            }
            "v" => check_keyword(word, "var", 1, TokenType::Var),
            "w" => check_keyword(word, "while", 1, TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.current() >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            tokens.push(self.scan_token());
        }

        tokens
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.token_start = self.current();
        let c = self.advance();
        if is_ident(c) {
            return self.identifier();
        }
        if is_digit(c) {
            return self.number_literal();
        }
        match c {
            None => Token::new(TokenType::EoF, "".to_string(), self.line),
            Some(c) => match c {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                ';' => self.make_token(TokenType::Semicolon),
                '/' => self.make_token(TokenType::Slash),
                '*' => self.make_token(TokenType::Star),
                '!' => {
                    if self.maybe_match('=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.maybe_match('=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.maybe_match('=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.maybe_match('=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                '"' => self.string_literal(),
                _ => {
                    TokenType::error(TokenType::UnexpectedCharacterError, self.line);
                    self.make_token(TokenType::UnexpectedCharacterError)
                }
            },
        }
    }

}
