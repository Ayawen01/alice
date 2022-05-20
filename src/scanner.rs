use crate::{token::{Token, TokenType, Literal}, error::AliceError};

pub struct Scanner {
    source: Vec<u8>,
    current: usize,
    line: u32
}

impl Scanner {
    #[inline]
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner { source, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<AliceError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut is_error = false;

        while !self.is_at_end() {
            let byte = self.advance();
            match byte {
                b'(' => tokens.push(Token{r#type: TokenType::LeftParen,     lexeme: None, literal: None, line: self.line}),
                b')' => tokens.push(Token{r#type: TokenType::RightParen,    lexeme: None, literal: None, line: self.line}),
                b'[' => tokens.push(Token{r#type: TokenType::LeftSquare,    lexeme: None, literal: None, line: self.line}),
                b']' => tokens.push(Token{r#type: TokenType::RightSquare,   lexeme: None, literal: None, line: self.line}),
                b'{' => tokens.push(Token{r#type: TokenType::LeftBrace,     lexeme: None, literal: None, line: self.line}),
                b'}' => tokens.push(Token{r#type: TokenType::RightBrace,    lexeme: None, literal: None, line: self.line}),
                b',' => tokens.push(Token{r#type: TokenType::Comma,         lexeme: None, literal: None, line: self.line}),
                b'.' => tokens.push(Token{r#type: TokenType::Dot,           lexeme: None, literal: None, line: self.line}),
                b'-' => tokens.push(Token{r#type: TokenType::Minus,         lexeme: None, literal: None, line: self.line}),
                b'+' => tokens.push(Token{r#type: TokenType::Plus,          lexeme: None, literal: None, line: self.line}),
                b';' => tokens.push(Token{r#type: TokenType::Semicolon,     lexeme: None, literal: None, line: self.line}),
                b'*' => tokens.push(Token{r#type: TokenType::Star,          lexeme: None, literal: None, line: self.line}),

                b'!' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    tokens.push(Token{r#type: token_type, lexeme: None, literal: None, line: self.line});
                }
                b'=' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::EqualEqual
                    } else if self.matching(b'>') {
                        TokenType::Arrowhead
                    } else {
                        TokenType::Equal
                    };
                    tokens.push(Token{r#type: token_type, lexeme: None, literal: None, line: self.line});
                }
                b'<' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    tokens.push(Token{r#type: token_type, lexeme: None, literal: None, line: self.line});
                }
                b'>' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    tokens.push(Token{r#type: token_type, lexeme: None, literal: None, line: self.line});
                }
                b'/' => {
                    if self.matching(b'/') {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        tokens.push(Token{r#type: TokenType::Slash, lexeme: None, literal: None, line: self.line});
                    }
                }

                b' ' |
                b'\r'|
                b'\t' => continue,
                b'\n' => self.line += 1,

                b'"' => {
                    match self.string() {
                        Ok(literal) => tokens.push(Token{r#type: TokenType::String, lexeme: None, literal, line: self.line}),
                        Err(e) => {
                            is_error = true;
                            errors.push(e);
                        }
                    }
                }

                _ => {
                    if self.is_digit(byte) {
                        match self.number() {
                            Ok((r#type, literal)) => {
                                tokens.push(Token{r#type, lexeme: None, literal, line: self.line});
                            }
                            Err(e) => {
                                is_error = true;
                                errors.push(e);
                            }
                        }
                    } else if self.is_alpha(byte) {
                        let (r#type, lexeme, literal) = self.identifier();
                        tokens.push(Token { r#type, lexeme, literal, line: self.line })
                    } else {
                        is_error = true;
                        errors.push(AliceError::SyntaxError(format!("unknown token '{}'.", byte as char).into(), self.line));
                    }
                }
            }
        }

        if is_error {
            return Err(errors);
        }

        tokens.push(Token {
            r#type: TokenType::Eof,
            lexeme: None,
            literal: None,
            line: self.line - 1
        });

        Ok(tokens)
    }

    fn string<'a>(&mut self) -> Result<Option<Literal>, AliceError> {
        let start_index = self.current;

        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(AliceError::SyntaxError("not a full string.".into(), self.line))
        }

        let str = String::from_utf8(self.source[start_index..self.current].to_vec()).unwrap();

        self.advance();

        Ok(Some(Literal::String(str)))
    }

    fn number<'a>(&mut self) -> Result<(TokenType, Option<Literal>), AliceError> {
        let mut is_double = false;
        let start_index = self.current - 1;
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            is_double = true;
            self.advance();
            
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num = String::from_utf8(self.source[start_index..self.current].to_vec()).unwrap();
        
        if is_double {
            let num = num.parse::<f64>().unwrap();
            Ok((TokenType::F64, Some(Literal::F64(num))))
        } else {
            let num = num.parse::<i64>().unwrap();
            Ok((TokenType::I64, Some(Literal::I64(num))))
        }
    }

    fn identifier<'a>(&mut self) -> (TokenType, Option<String>, Option<Literal>) {
        let start_index = self.current - 1;
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let id = String::from_utf8(self.source[start_index..self.current].to_vec()).unwrap();
        match id.as_str() {
            "and"       =>    (TokenType::And,      None, None),
            "or"        =>    (TokenType::Or,       None, None),
            "if"        =>    (TokenType::If,       None, None),
            "else"      =>    (TokenType::Else,     None, None),
            "true"      =>    (TokenType::True,     None, None),
            "false"     =>    (TokenType::False,    None, None),
            "fn"        =>    (TokenType::Fn,       None, None),
            "let"       =>    (TokenType::Let,      None, None),
            "nil"       =>    (TokenType::Nil,      None, None),
            "println"   =>    (TokenType::Println,  None, None),
            "return"    =>    (TokenType::Return,   None, None),
            "for"       =>    (TokenType::For,      None, None),
            "in"        =>    (TokenType::In,       None, None),
            _ => {
                (TokenType::Identifier, Some(id.clone()), Some(Literal::Id(id)))
            }
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    #[inline]
    fn advance(&mut self) -> u8 {
        self.current += 1;
        *self.source.get(self.current - 1).unwrap()
    }

    #[inline]
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0'
        }

        *self.source.get(self.current).unwrap()
    }

    #[inline]
    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0'
        }

        *self.source.get(self.current + 1).unwrap()
    }

    #[inline]
    fn matching(&mut self, byte: u8) -> bool {
        if self.is_at_end() {
            return false
        }

        if self.peek() != byte {
            return false
        }

        self.current += 1;
        true
    }

    #[inline]
    fn is_alpha_numeric(&self, c: u8) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    #[inline]
    fn is_digit(&self, c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    #[inline]
    fn is_alpha(&self, c: u8) -> bool {
        c >= b'a' && c <= b'z' ||
        c >= b'A' && c <= b'Z' ||
        c == b'_'
    }
}