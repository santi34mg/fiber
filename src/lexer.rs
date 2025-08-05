use crate::token::{Keyword, Operator, Token};

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
    fn bump(&mut self) -> Option<char> {
        if self.position > self.input.len() {
            return None;
        }
        let c = self.peek()?;
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
        self.input[self.position..].chars().next() // This gets the next of the iterator built by
                                                   // .chars(), I believe.
    }

    fn lex_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let c = self.peek()?;
        match c {
            '+' => {
                self.bump();
                Some(Token::Operator(Operator::Plus))
            } // Example of + operator
            '=' => {
                self.bump();
                Some(Token::Operator(Operator::Assign))
            },
            c if c.is_ascii_digit() => {
                let start = self.position;
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.bump();
                    } else {
                        break;
                    }
                }
                let num_str = &self.input[start..self.position];
                let value = num_str.parse::<i64>().expect("Non number value");
                Some(Token::Number(value))
            }
            c if c.is_alphabetic() => {
                let start = self.position;
                while let Some(c) = self.peek() {
                    if c.is_alphanumeric() {
                        self.bump();
                    } else {
                        break;
                    }
                }
                let identifier = &self.input[start..self.position];
                match identifier {
                    "let" => Some(Token::Keyword(Keyword::Let)),
                    "fn" => Some(Token::Keyword(Keyword::Fn)),
                    _ => Some(Token::Identifier(identifier.to_string()))
                }
            }
            _ => None,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_token()
    }
}
