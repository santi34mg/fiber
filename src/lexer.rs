use crate::token::{Keyword, Operator, Punctuation, Token};

pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
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
        self.input[self.position..].chars().next()
    }

    fn lex_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let c = self.peek()?;
        self.bump();
        match c {
            '=' => Some(self.lex_eq()),
            '+' => Some(Token::Operator(Operator::Plus)),
            '*' => Some(Token::Operator(Operator::Multply)),
            '/' => Some(self.lex_slash()),
            '-' => Some(Token::Operator(Operator::Minus)),
            '(' => Some(Token::Punctuation(Punctuation::OpenParen)),
            ')' => Some(Token::Punctuation(Punctuation::CloseParen)),
            '{' => Some(Token::Punctuation(Punctuation::OpenCurly)),
            '}' => Some(Token::Punctuation(Punctuation::CloseCurly)),
            ',' => Some(Token::Punctuation(Punctuation::Comma)),
            ';' => Some(Token::Punctuation(Punctuation::Semicolon)),
            ':' => Some(Token::Punctuation(Punctuation::Colon)),
            c if c.is_ascii_digit() => Some(self.lex_number()),
            c if c.is_alphabetic() => Some(self.lex_identifier()),
            _ => Some(Token::Unkown(c)),
        }
    }

    fn lex_eq(&mut self) -> Token {
        if let Some(c) = self.peek() {
            if c == '=' {
                self.bump();
                return Token::Operator(Operator::Equals);
            }
        }
        Token::Operator(Operator::Assign)
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

    fn lex_slash(&mut self) -> Token {
        if let Some(c) = self.peek() {
            if c == '/' {
                self.skip_line()
            }
            return Token::Comment;
        }
        Token::Operator(Operator::Divide)
    }

    fn lex_number(&mut self) -> Token {
        let start = self.position - 1; // we bumped before match
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
        }
        let num_str = &self.input[start..self.position];
        let value = num_str.parse::<i64>().expect("Non number value");
        Token::NumberLiteral(value)
    }

    fn lex_identifier(&mut self) -> Token {
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
            "let" => Token::Keyword(Keyword::Let),
            "fn" => Token::Keyword(Keyword::Fn),
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "while" => Token::Keyword(Keyword::While),
            "return" => Token::Keyword(Keyword::Return),
            "true" => Token::BooleanLiteral(true),
            "false" => Token::BooleanLiteral(false),
            _ => Token::Identifier(name.to_string()),
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
