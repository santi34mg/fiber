#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        return Self { kind, line, column };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    TypeIdentifier(TypeIdentifier),
    NumberLiteral(i32),
    BooleanLiteral(bool),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
    Unkown(char),
    Comment,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeIdentifier {
    Number,
    Boolean,
    UserDefinedType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Var,
    If,
    Else,
    While,
    Return,
    Func,
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
    // Logical operators
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Comma,
    Colon,
}
