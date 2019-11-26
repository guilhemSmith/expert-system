#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Operand {
    negated: bool,
    symbol: char,
}

impl Operand {
    pub fn new(negated: bool, symbol: char) -> Self {
        Operand { negated, symbol }
    }

    pub fn negated(&self) -> bool {
        self.negated
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn display_str(&self) -> String {
        format!("{}", self.symbol())
    }
}
