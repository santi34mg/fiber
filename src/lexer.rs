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
            '+' => Some(Token::Operator(Operator::Plus)),
            '-' => {
                if let Some(c) = self.peek() {
                    if c == '>' {
                        self.bump();
                        return Some(Token::Punctuation(Punctuation::ThinArrow));
                    }
                }
                Some(Token::Operator(Operator::Minus))
            }
            '=' => {
                if let Some(c) = self.peek() {
                    if c == '=' {
                        self.bump();
                        return Some(Token::Operator(Operator::Equals));
                    }
                }
                Some(Token::Operator(Operator::Assign))
            }
            '(' => Some(Token::Punctuation(Punctuation::OpenParen)),
            ')' => Some(Token::Punctuation(Punctuation::CloseParen)),
            '{' => Some(Token::Punctuation(Punctuation::OpenCurly)),
            '}' => Some(Token::Punctuation(Punctuation::CloseCurly)),
            ',' => Some(Token::Punctuation(Punctuation::Comma)),
            ';' => Some(Token::Punctuation(Punctuation::Semicolon)),
            c if c.is_ascii_digit() => {
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
                Some(Token::NumberLiteral(value))
            }
            c if c.is_alphabetic() => {
                let start = self.position - 1; // we bumped before match
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
                    "if" => Some(Token::Keyword(Keyword::If)),
                    "else" => Some(Token::Keyword(Keyword::Else)),
                    "while" => Some(Token::Keyword(Keyword::While)),
                    "return" => Some(Token::Keyword(Keyword::Return)),
                    "true" => Some(Token::BooleanLiteral(true)),
                    "false" => Some(Token::BooleanLiteral(false)),
                    _ => Some(Token::Identifier(identifier.to_string())),
                }
            }
            _ => Some(Token::Unkown(c)),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex_token();
        // println!("{:?}", token);
        token
    }
}
