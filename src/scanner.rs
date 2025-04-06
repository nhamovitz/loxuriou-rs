pub(crate) struct Scanner<'a> {
    line: usize,
    start_char: usize,
    next_char: usize,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    ate_char: Option<(usize, char)>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Scanner {
            line: 1,
            start_char: 0,
            next_char: 0,
            chars: source.char_indices().peekable(),
            ate_char: None,
        }
    }

    pub(crate) fn scan_token(&mut self) -> Token {
        println!("{:?} {:?}", self.start_char, self.next_char);

        self.skip_whitespace();

        println!("skipped whitespace");

        self.start_char = self.next_char;

        let c = match self.ate_char {
            Some((i, c)) => {
                self.next_char = i + 1;
                self.ate_char = None;
                c
            }
            None => match self.advance() {
                Some(c) => c,
                None => return self.make_token(TokenType::EOF),
            },
        };

        match c {
            '(' => self.make_token(TokenType::LEFT_PAREN),
            ')' => self.make_token(TokenType::RIGHT_PAREN),
            '{' => self.make_token(TokenType::LEFT_BRACE),
            '}' => self.make_token(TokenType::RIGHT_BRACE),
            ';' => self.make_token(TokenType::SEMICOLON),
            ',' => self.make_token(TokenType::COMMA),
            '.' => self.make_token(TokenType::DOT),
            '-' => self.make_token(TokenType::MINUS),
            '+' => self.make_token(TokenType::PLUS),
            '/' => self.make_token(TokenType::SLASH),
            '*' => self.make_token(TokenType::STAR),

            '!' => {
                if self.match_next('=') {
                    self.make_token(TokenType::BANG_EQUAL)
                } else {
                    self.make_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.make_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.make_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.make_token(TokenType::LESS_EQUAL)
                } else {
                    self.make_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.make_token(TokenType::GREATER_EQUAL)
                } else {
                    self.make_token(TokenType::GREATER)
                }
            }
            '"' => self.make_string(),
            '0'..='9' => self.make_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.make_identifier(c),

            _ => self.error_token("Unexpected character."),
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        match self.chars.peek() {
            Some(&(_, c)) => {
                if c == expected {
                    _ = self.chars.next();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.peek() {
            Some(_) => {
                let (i, char) = self.chars.next().expect("we peeked and found Some");
                self.next_char = i + 1;
                Some(char)
            }
            None => None,
        }
    }

    fn make_token<'b>(&self, token_type: TokenType<'b>) -> Token<'b> {
        Token {
            token_type,
            start: self.start_char,
            length: self.next_char - self.start_char,
            line: self.line,
        }
    }

    fn error_token<'b>(&self, error_message: &'b str) -> Token<'b> {
        Token {
            token_type: TokenType::ERROR(error_message),
            start: 0,
            length: error_message.len(),
            line: self.line,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                }
                _ = self.chars.next();
            } else if c == '/' {
                let slash = self.chars.next().expect("we peeked and found Some");
                assert_eq!('/', slash.1);
                self.ate_char = Some((slash.0, '/'));

                if let Some(&(_, c)) = self.chars.peek() {
                    if c == '/' {
                        // found a comment

                        while let Some(&(_, c)) = self.chars.peek() {
                            if c == '\n' {
                                break;
                            } else {
                                let _ = self.advance();
                            }
                        }
                    } else {
                    }
                } else {
                }
            } else {
                // Neither whitespace nor a '/'
                break;
            }
        }
    }

    fn make_string(&mut self) -> Token<'_> {
        while let Some(&(_, c)) = self.chars.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            _ = self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // Consume the closing quote
        _ = self.advance();
        self.make_token(TokenType::STRING)
    }

    fn make_number(&mut self) -> Token<'_> {
        'outer: while let Some(&(_, c)) = self.chars.peek() {
            match c {
                '0'..='9' => {
                    _ = self.advance();
                }
                '.' => {
                    let dot = self.chars.next().expect("we peeked and found Some");
                    assert_eq!(dot.1, '.');

                    let after_dot = self.chars.peek();
                    match after_dot {
                        Some((_, '0'..='9')) => {
                            while let Some(&(_, c)) = self.chars.peek() {
                                if c.is_ascii_digit() {
                                    _ = self.advance();
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        _ => {
                            self.ate_char = Some(dot);
                            break; // technically unnecessary; for clarity
                        }
                    }

                    break;
                }
                _ => break,
            }
        }

        self.make_token(TokenType::NUMBER)
    }

    fn make_identifier(&mut self, first_char: char) -> Token<'_> {
        let mut ident = vec![first_char];
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                let c = self.advance().expect("we peeked and found Some");
                ident.push(c);
            } else {
                break;
            }
        }

        self.make_token(self.find_identifier_type(&ident))
    }

    fn find_identifier_type(&self, ident: &[char]) -> TokenType<'_> {
        println!("finding type of identifier: {:?}", ident);

        let (first, rest) = (ident[0], &ident[1..]);
        match first {
            'a' => self.check_keyword(rest, "nd", TokenType::AND),
            'c' => self.check_keyword(rest, "lass", TokenType::CLASS),
            'e' => self.check_keyword(rest, "lse", TokenType::ELSE),
            'i' => self.check_keyword(rest, "f", TokenType::IF),
            'n' => self.check_keyword(rest, "il", TokenType::NIL),
            'o' => self.check_keyword(rest, "r", TokenType::OR),
            'p' => self.check_keyword(rest, "rint", TokenType::PRINT),
            'r' => self.check_keyword(rest, "eturn", TokenType::RETURN),
            's' => self.check_keyword(rest, "uper", TokenType::SUPER),
            'v' => self.check_keyword(rest, "ar", TokenType::VAR),
            'w' => self.check_keyword(rest, "hile", TokenType::WHILE),

            'f' if rest.len() > 0 => {
                let (first, rest) = (rest[0], &rest[1..]);
                match first {
                    'a' => self.check_keyword(rest, "lse", TokenType::FALSE),
                    'o' => self.check_keyword(rest, "r", TokenType::FOR),
                    'u' => self.check_keyword(rest, "n", TokenType::FUN),
                    _ => TokenType::IDENTIFIER,
                }
            }
            't' if rest.len() > 0 => {
                let (first, rest) = (rest[0], &rest[1..]);
                match first {
                    'h' => self.check_keyword(rest, "is", TokenType::THIS),
                    'r' => self.check_keyword(rest, "ue", TokenType::TRUE),
                    _ => TokenType::IDENTIFIER,
                }
            }

            _ => TokenType::IDENTIFIER,
        }
    }

    fn check_keyword<'b>(
        &self,
        rest: &[char],
        expected_finish: &str,
        token_type: TokenType<'b>,
    ) -> TokenType<'b> {
        if rest
            .iter()
            .zip(expected_finish.chars())
            .all(|(&c1, c2)| c1 == c2)
        {
            token_type
        } else {
            TokenType::IDENTIFIER
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) enum TokenType<'a> {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    ERROR(&'a str),
    EOF,
}

pub(crate) struct Token<'a> {
    pub(crate) token_type: TokenType<'a>,
    pub(crate) start: usize,
    pub(crate) length: usize,
    pub(crate) line: usize,
}
