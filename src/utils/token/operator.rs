use crate::utils::error::{ESError, ESErrorKind};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    And,
    Or,
    Xor,
    Ind,
    Deind,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    negated: bool,
    opc: OpCode,
}

impl Operator {
    pub fn op_from_char(c: char) -> Result<OpCode, ESError> {
        match c {
            '+' => Ok(OpCode::And),
            '|' => Ok(OpCode::Or),
            '^' => Ok(OpCode::Xor),
            '(' => Ok(OpCode::Ind),
            ')' => Ok(OpCode::Deind),
            _ => Err(ESError::new_w_what(
                ESErrorKind::UnknownOp,
                format!("Unknown operator {}", c),
            )),
        }
    }

    pub fn op_to_char(opc: OpCode) -> char {
        match opc {
            OpCode::And => '+',
            OpCode::Or => '|',
            OpCode::Xor => '^',
            OpCode::Ind => '(',
            OpCode::Deind => ')',
        }
    }

    pub fn op_code(&self) -> OpCode {
        self.opc
    }

    pub fn new(negated: bool, opc: OpCode) -> Self {
        Operator { negated, opc }
    }

    pub fn new_from_char(negated: bool, opc: char) -> Result<Self, ESError> {
        let nopc = Operator::op_from_char(opc)?;
        Ok(Operator { negated, opc: nopc })
    }

    pub fn negated(&self) -> bool {
        self.negated
    }

    pub fn display_str(&self) -> String {
        format!("{}", Operator::op_to_char(self.op_code()))
    }
}
