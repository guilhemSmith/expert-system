use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use super::{FactValue, Graph};
use crate::utils::{
    error::{ESError, ESResult},
    token::{OpCode, Operand, Operator, Token},
};

pub struct Rule {
    facts_id: HashSet<char>,
    condition: Vec<Token>,
}

impl Rule {
    pub fn new(condition: Vec<Token>, facts_id: HashSet<char>) -> Self {
        Rule {
            facts_id,
            condition,
        }
    }

    pub fn facts_id(&self) -> HashSet<char> {
        self.facts_id.clone()
    }

    pub fn process(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, FactValue>,
    ) -> ESResult<FactValue> {
        let stack = (&self.condition[..]).to_vec();
        let (res, stack) = self.process_stack(graph, seen, stack)?;
        if stack.len() == 0 {
            Ok(res)
        } else {
            Err(ESError::corrupted_rule())
        }
    }

    fn process_stack(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, FactValue>,
        mut stack: Vec<Token>,
    ) -> ESResult<(FactValue, Vec<Token>)> {
        let token = stack.pop();
        match token {
            None => Err(ESError::corrupted_rule()),
            Some(first) => match first {
                Token::Factual(op) => {
                    let res = self.get_fact(graph, seen, &op)?;
                    Ok((res, stack))
                }
                Token::Computable(op) => {
                    let (res, stack) = self.compute(graph, seen, op, stack)?;
                    Ok((res, stack))
                }
                _ => Err(ESError::corrupted_rpn_stack()),
            },
        }
    }

    fn compute(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, FactValue>,
        operator: Operator,
        stack: Vec<Token>,
    ) -> ESResult<(FactValue, Vec<Token>)> {
        let (first_op, stack) = self.process_stack(graph, seen, stack)?;
        let (second_op, stack) = self.process_stack(graph, seen, stack)?;
        let mut res = match operator.op_code() {
            OpCode::And => and(first_op, second_op),
            OpCode::Or => or(first_op, second_op),
            OpCode::Xor => xor(first_op, second_op),
        };
        if operator.negated() {
            res = match res {
                FactValue::Fixed(val) => FactValue::Fixed(!val),
                _ => FactValue::Absurd,
            }
        }
        return Ok((res, stack));
    }

    fn get_fact(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, FactValue>,
        op: &Operand,
    ) -> ESResult<FactValue> {
        let solution: FactValue;
        if let Some(computed) = seen.get(&op.symbol()) {
            solution = *computed;
        } else {
            seen.insert(op.symbol(), FactValue::Undefined);
            solution = graph.try_fact(op.symbol(), seen)?;
            seen.insert(op.symbol(), solution);
        }
        return match solution {
            FactValue::Fixed(val) => {
                if op.negated() {
                    Ok(FactValue::Fixed(!val))
                } else {
                    Ok(FactValue::Fixed(val))
                }
            }
            FactValue::Undefined => {
                if !op.negated() {
                    Ok(FactValue::Undefined)
                } else {
                    Ok(FactValue::Absurd)
                }
            }
            FactValue::Absurd => Ok(FactValue::Absurd),
        };
    }
}

fn or(left: FactValue, right: FactValue) -> FactValue {
    match (left, right) {
        (FactValue::Fixed(true), _) => FactValue::Fixed(true),
        (_, FactValue::Fixed(true)) => FactValue::Fixed(true),
        (FactValue::Absurd, _) => FactValue::Absurd,
        (_, FactValue::Absurd) => FactValue::Absurd,
        _ => FactValue::Fixed(false),
    }
}

fn and(left: FactValue, right: FactValue) -> FactValue {
    match (left, right) {
        (FactValue::Fixed(true), FactValue::Fixed(true)) => {
            FactValue::Fixed(true)
        }
        (FactValue::Fixed(false), _) => FactValue::Fixed(false),
        (_, FactValue::Fixed(false)) => FactValue::Fixed(false),
        (FactValue::Absurd, _) => FactValue::Absurd,
        (_, FactValue::Absurd) => FactValue::Absurd,
        _ => FactValue::Fixed(false),
    }
}

fn xor(left: FactValue, right: FactValue) -> FactValue {
    match (left, right) {
        (FactValue::Absurd, _) => FactValue::Absurd,
        (_, FactValue::Absurd) => FactValue::Absurd,
        (FactValue::Fixed(false), FactValue::Fixed(false)) => {
            FactValue::Fixed(false)
        }
        (FactValue::Fixed(true), FactValue::Fixed(true)) => {
            FactValue::Fixed(false)
        }
        (FactValue::Fixed(true), _) => FactValue::Fixed(true),
        (_, FactValue::Fixed(true)) => FactValue::Fixed(true),
        _ => FactValue::Fixed(false),
    }
}
