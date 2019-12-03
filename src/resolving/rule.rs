use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use super::Graph;
use crate::utils::{
    error::{ESError, ESResult},
    token::{OpCode, Operator, Token},
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
        seen: &mut HashMap<char, Option<bool>>,
    ) -> ESResult<bool> {
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
        seen: &mut HashMap<char, Option<bool>>,
        mut stack: Vec<Token>,
    ) -> ESResult<(bool, Vec<Token>)> {
        let token = stack.pop();
        match token {
            None => Err(ESError::corrupted_rule()),
            Some(first) => match first {
                Token::Solved(value) => Ok((value, stack)),
                Token::Factual(op) => {
                    let mut res = self.get_fact(graph, seen, op.symbol())?;
                    if op.negated() {
                        res = !res;
                    }
                    Ok((res, stack))
                }
                Token::Computable(op) => {
                    let (mut res, stack) =
                        self.compute(graph, seen, op, stack)?;
                    if op.negated() {
                        res = !res;
                    }
                    Ok((res, stack))
                }
                _ => Err(ESError::corrupted_rpn_stack()),
            },
        }
    }

    fn compute(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        operator: Operator,
        stack: Vec<Token>,
    ) -> ESResult<(bool, Vec<Token>)> {
        let (first_op, stack) = self.process_stack(graph, seen, stack)?;
        let (second_op, stack) = self.process_stack(graph, seen, stack)?;
        let res = match operator.op_code() {
            OpCode::And => first_op && second_op,
            OpCode::Or => first_op || second_op,
            OpCode::Xor => (first_op || second_op) && !(first_op && second_op),
        };
        return Ok((res, stack));
    }

    fn get_fact(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        symbol: char,
    ) -> ESResult<bool> {
        if let Some(computed) = seen.get(&symbol) {
            return match computed {
                None => Ok(false),
                Some(res) => return Ok(*res),
            };
        }
        seen.insert(symbol, None);
        let res = graph.try_fact(symbol, seen)?;
        seen.insert(symbol, Some(res));
        return Ok(res);
    }
}
