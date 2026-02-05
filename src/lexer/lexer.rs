use core::panic;
use std::char;

use crate::token::{Keyword, Literal, Operator, Punctuation, Token, TokenKind, TypeIdentifier};

pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += c.len_utf8();
        Some(c)
    }

    fn skip_while<F>(&mut self, mut pred: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(|c| c.is_whitespace());
    }

    fn lex_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let start_line = self.line;
        let start_col = self.column;

        let c = self.peek()?;
        let kind: Option<TokenKind> = match c {
            '=' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::Equals))
                } else {
                    Some(TokenKind::Operator(Operator::Assign))
                }
            }
            '!' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::Different))
                } else {
                    Some(TokenKind::Operator(Operator::Not))
                }
            }
            '>' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::GreaterEqual))
                } else {
                    Some(TokenKind::Operator(Operator::GreaterThan))
                }
            }
            '<' => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::LesserEqual))
                } else {
                    Some(TokenKind::Operator(Operator::LesserThan))
                }
            }
            '+' => {
                self.bump();
                if self.peek() == Some('+') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::Increment))
                } else {
                    Some(TokenKind::Operator(Operator::Plus))
                }
            }
            '-' => {
                self.bump();
                if self.peek() == Some('-') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::Decrement))
                } else {
                    Some(TokenKind::Operator(Operator::Minus))
                }
            }
            '*' => {
                self.bump();
                Some(TokenKind::Operator(Operator::Multiply))
            }
            '/' => {
                self.bump();
                if self.peek() == Some('/') {
                    self.bump();
                    self.skip_while(|c| c != '\n');
                    None
                } else {
                    Some(TokenKind::Operator(Operator::Divide))
                }
            }
            '&' => {
                self.bump();
                if self.peek() == Some('&') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::And))
                } else {
                    Some(TokenKind::Unknown('&'))
                }
            }
            '|' => {
                self.bump();
                if self.peek() == Some('|') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::Or))
                } else {
                    Some(TokenKind::Unknown('|'))
                }
            }
            '(' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::OpenParen))
            }
            ')' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::CloseParen))
            }
            '{' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::OpenCurly))
            }
            '}' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::CloseCurly))
            }
            ',' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::Comma))
            }
            ';' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::Semicolon))
            }
            ':' => {
                self.bump();
                Some(TokenKind::Punctuation(Punctuation::Colon))
            }
            '\'' => {
                self.bump(); // consume opening quote
                let ch = self.bump()?; // get the character
                if self.bump()? != '\'' {
                    // expect closing quote
                    return Some(Token::new(TokenKind::Unknown(ch), start_line, start_col));
                }
                Some(TokenKind::Literal(Literal::Character(ch)))
            }
            c if c.is_ascii_digit() => {
                let num = self.lex_numeric(c);
                num
            }
            c if c.is_alphabetic() => Some(self.lex_identifier_or_keyword()),
            c => {
                self.bump();
                Some(TokenKind::Unknown(c))
            }
        };
        Some(Token::new(kind?, start_line, start_col))
    }

    fn lex_numeric(&mut self, first: char) -> Option<TokenKind> {
        let mut start = self.position;
        // first char is 0 might be 0x... or might be 0123
        let (base, f): (u32, fn(char) -> bool) = if first == '0' {
            self.bump();
            let second = self.peek()?;
            match second {
                'x' => {
                    self.bump();
                    start = self.position;
                    (16, |c: char| c.is_ascii_hexdigit())
                }
                'b' => {
                    self.bump();
                    start = self.position;
                    (2, |c: char| c == '0' || c == '1')
                }
                'd' => {
                    self.bump();
                    (10, |c: char| c.is_ascii_digit())
                }
                c if c.is_ascii_digit() => (10, |c: char| c.is_ascii_digit() || c == '.'),
                '.' => (10, |c: char| c.is_ascii_digit() || c == '.'),
                // TODO: better error handling
                _ => {
                    panic!()
                }
            }
        } else {
            (10, |c: char| c.is_ascii_digit() || c == '.')
        };
        self.skip_while(|c| f(c));
        let num_str = &self.input[start..self.position];
        if num_str.contains('.') {
            let value = ("0".to_string() + num_str).parse::<f32>().ok()?;
            return Some(TokenKind::Literal(Literal::Float(value)));
        } else {
            let value = u32::from_str_radix(num_str, base).unwrap_or_else(|e| {
                eprintln!("Error: {}\nfor string \"{}\"", e, num_str);
                // TODO: better errors
                panic!();
            });
            return Some(TokenKind::Literal(Literal::Integer(value)));
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> TokenKind {
        let start = self.position;
        self.skip_while(|c| c.is_alphanumeric() || c == '_');
        let name = &self.input[start..self.position];
        match name {
            "let" => TokenKind::Keyword(Keyword::Let),
            "function" => TokenKind::Keyword(Keyword::Function),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "for" => TokenKind::Keyword(Keyword::For),
            "return" => TokenKind::Keyword(Keyword::Return),
            "int" => TokenKind::TypeIdentifier(TypeIdentifier::Number),
            "bool" => TokenKind::TypeIdentifier(TypeIdentifier::Boolean),
            "char" => TokenKind::TypeIdentifier(TypeIdentifier::Char),
            "true" => TokenKind::Literal(Literal::Boolean(true)),
            "false" => TokenKind::Literal(Literal::Boolean(false)),
            _ => TokenKind::Identifier(name.to_string()),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_token()
    }
}
