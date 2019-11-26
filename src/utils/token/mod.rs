mod modifier;
mod operand;
mod operator;

pub use modifier::{Modifier, ModifierType};
pub use operand::Operand;
pub use operator::{OpCode, Operator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Behavioral(Modifier),
    Computable(Operator),
    Factual(Operand),
    Solved(bool),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Token::Computable(op) => write!(f, "{}", op.display_str()),
            &Token::Factual(op) => write!(f, "{}", op.display_str()),
            &Token::Behavioral(op) => write!(f, "{}", op.display_str()),
            &Token::Solved(op) => write!(f, "{}", op),
        }
    }
}
