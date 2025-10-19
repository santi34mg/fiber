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