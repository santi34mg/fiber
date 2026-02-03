use crate::token::keyword::Keyword;
use crate::token::operator::Operator;
use crate::token::punctuation::Punctuation;
use crate::token::type_identifier::TypeIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    TypeIdentifier(TypeIdentifier),
    NumberLiteral(i32),
    BooleanLiteral(bool),
    CharLiteral(char),
    Keyword(Keyword),
    Operator(Operator),
    Punctuation(Punctuation),
    Unkown(char),
}