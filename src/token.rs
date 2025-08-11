
#[derive(Debug)]
pub enum Token {
    Identifier(String),
    NumberLiteral(i64),
    BooleanLiteral(bool),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
    Unkown(char),
    Comment,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
    If,
    Else,
    While,
    Return,
    Fn,
}


#[derive(Debug)]
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

#[derive(Debug)]
pub enum Punctuation {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    ThinArrow,
    Comma,
    Colon,
}
