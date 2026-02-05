#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(u32),
    Float(f32),
    Boolean(bool),
    Character(char),
}
