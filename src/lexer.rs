use crate::token::{Keyword, Operator, Punctuation, Token, TokenKind};

pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0, line: 0, column: 0 }
    }

    fn bump(&mut self) -> Option<char> {
        if self.position > self.input.len() {
            return None;
        }
        let c = self.peek()?;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += c.len_utf8();
        }
        self.position += c.len_utf8();
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn lex_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let c = self.peek()?;
        self.bump();
        let kind = match c {
            '=' => self.lex_eq(),
            '+' => TokenKind::Operator(Operator::Plus),
            '*' => TokenKind::Operator(Operator::Multply),
            '/' => self.lex_slash().unwrap(),
            '-' => TokenKind::Operator(Operator::Minus),
            '(' => TokenKind::Punctuation(Punctuation::OpenParen),
            ')' => TokenKind::Punctuation(Punctuation::CloseParen),
            '{' => TokenKind::Punctuation(Punctuation::OpenCurly),
            '}' => TokenKind::Punctuation(Punctuation::CloseCurly),
            ',' => TokenKind::Punctuation(Punctuation::Comma),
            ';' => TokenKind::Punctuation(Punctuation::Semicolon),
            ':' => TokenKind::Punctuation(Punctuation::Colon),
            c if c.is_ascii_digit() => self.lex_number(),
            c if c.is_alphabetic() => self.lex_identifier(),
            _ => TokenKind::Unkown(c),
        };
        Some(Token::new(kind, self.line, self.column))
    }

    fn lex_eq(&mut self) -> TokenKind {
        if let Some(c) = self.peek() {
            if c == '=' {
                self.bump();
                return TokenKind::Operator(Operator::Equals);
            }
        }
        TokenKind::Operator(Operator::Assign)
    }

    fn skip_line(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            } else {
                self.bump();
            }
        }
    }

    fn lex_slash(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            if c == '/' {
                self.skip_line();
                return Some(TokenKind::Comment);
            }
            Some(TokenKind::Operator(Operator::Divide))
        } else {
            None
        }
    }

    fn lex_number(&mut self) -> TokenKind {
        let start = self.position - 1; // we bumped before match
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
        }
        let num_str = &self.input[start..self.position];
        let value = num_str.parse::<i32>().expect("Non number value");
        TokenKind::NumberLiteral(value)
    }

    fn lex_identifier(&mut self) -> TokenKind {
        let start = self.position - 1; // we bumped before match
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.bump();
            } else {
                break;
            }
        }
        let name = &self.input[start..self.position];
        match name {
            "var" => TokenKind::Keyword(Keyword::Var),
            "fn" => TokenKind::Keyword(Keyword::Fn),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "while" => TokenKind::Keyword(Keyword::While),
            "return" => TokenKind::Keyword(Keyword::Return),
            "true" => TokenKind::BooleanLiteral(true),
            "false" => TokenKind::BooleanLiteral(false),
            _ => TokenKind::Identifier(name.to_string()),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex_token();
        token
    }
}
