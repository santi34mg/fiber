
#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Keyword(Keyword),
    Operator(Operator),
    Unkown(char),
    EOF,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
    Fn,
    If,
    Else,
    // More and more
}


#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multply,
    Divide,
    Assign,
}
