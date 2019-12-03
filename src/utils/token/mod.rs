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

impl Token {
    pub fn priority(&self) -> u8 {
        match self {
            Token::Computable(x) => match x.op_code() {
                OpCode::And | OpCode::Xor => 2,
                OpCode::Or => 1,
            },
            _ => 0,
        }
    }

    pub fn negated(&self) -> bool {
        match self {
            &Token::Computable(op) => op.negated(),
            &Token::Factual(op) => op.negated(),
            &Token::Behavioral(op) => op.negated(),
            &Token::Solved(_op) => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &Token::Computable(op) => op.display_str(),
            &Token::Factual(op) => op.display_str(),
            &Token::Behavioral(op) => op.display_str(),
            &Token::Solved(op) => op.to_string(),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
