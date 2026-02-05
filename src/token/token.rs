use crate::token::keyword::Keyword;
use crate::token::literal::Literal;
use crate::token::operator::Operator;
use crate::token::punctuation::Punctuation;
use crate::token::type_identifier::TypeIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    TypeIdentifier(TypeIdentifier),
    Literal(Literal),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
    Unknown(char),
}
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
