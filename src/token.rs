// TODO: tokens should probably store line and column numbers

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        return Self {
            kind, line, column,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    NumberLiteral(i32),
    BooleanLiteral(bool),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
    Unkown(char),
    Comment,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Else,
    While,
    Return,
    Fn,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Multply,
    Divide,
    // Boolean opeartor
    Equals,
    Different,
    GreaterThan,
    LesserThan,
    GreaterEqual,
    LesserEqual,
    // Assignments
    Assign,
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Comma,
    Colon,
}
