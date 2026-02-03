use crate::token::{Keyword, Operator, Punctuation, Token, TokenKind, TypeIdentifier};

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
                Some(TokenKind::Operator(Operator::Multply))
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
                    Some(TokenKind::Unkown('&'))
                }
            }
            '|' => {
                self.bump();
                if self.peek() == Some('|') {
                    self.bump();
                    Some(TokenKind::Operator(Operator::Or))
                } else {
                    Some(TokenKind::Unkown('|'))
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
                    return Some(Token::new(
                        TokenKind::Unkown(ch),
                        start_line,
                        start_col,
                    ));
                }
                Some(TokenKind::CharLiteral(ch))
            }
            c if c.is_ascii_digit() => Some(self.lex_number()),
            c if c.is_alphabetic() => Some(self.lex_identifier_or_keyword()),
            c => {
                self.bump();
                Some(TokenKind::Unkown(c))
            }
        };
        Some(Token::new(kind?, start_line, start_col))
    }

    fn lex_number(&mut self) -> TokenKind {
        let start = self.position;
        self.skip_while(|c| c.is_ascii_digit());
        let num_str = &self.input[start..self.position];
        let value = num_str.parse::<i32>().unwrap_or(0);
        TokenKind::NumberLiteral(value)
    }

    fn lex_identifier_or_keyword(&mut self) -> TokenKind {
        let start = self.position;
        self.skip_while(|c| c.is_alphanumeric() || c == '_');
        let name = &self.input[start..self.position];
        match name {
            "var" => TokenKind::Keyword(Keyword::Var),
            "func" => TokenKind::Keyword(Keyword::Func),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "while" => TokenKind::Keyword(Keyword::While),
            "return" => TokenKind::Keyword(Keyword::Return),
            "int" => TokenKind::TypeIdentifier(TypeIdentifier::Number),
            "bool" => TokenKind::TypeIdentifier(TypeIdentifier::Boolean),
            "char" => TokenKind::TypeIdentifier(TypeIdentifier::Char),
            "true" => TokenKind::BooleanLiteral(true),
            "false" => TokenKind::BooleanLiteral(false),
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
