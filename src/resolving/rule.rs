use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use super::Graph;
use crate::utils::{
    error::{ESError, ESResult},
    token::{OpCode, Token},
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
        self.process_stack(graph, seen, stack)
    }

    fn process_stack(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        mut stack: Vec<Token>,
    ) -> ESResult<bool> {
        match stack.pop() {
            None => return Err(ESError::corrupted_rule()),
            Some(first) => match stack.pop() {
                None => return self.single_token(graph, seen, first),
                Some(sec) => {
                    self.dual_token(graph, seen, (first, sec), &mut stack)?
                }
            },
        };
        self.process_stack(graph, seen, stack)
    }

    fn single_token(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        token: Token,
    ) -> ESResult<bool> {
        match token {
            Token::Factual(operand) => {
                let mut result =
                    self.get_fact(graph, seen, operand.symbol())?;
                if operand.negated() {
                    result = !result;
                }
                return Ok(result);
            }
            Token::Solved(val) => Ok(val),
            _ => return Err(ESError::corrupted_rpn_stack()),
        }
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

    fn dual_token(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        arg: (Token, Token),
        stack: &mut Vec<Token>,
    ) -> ESResult<()> {
        match stack.pop() {
            None => return Err(ESError::corrupted_rpn_stack()),
            Some(third) => {
                if let Token::Computable(operator) = third {
                    let mut result = match operator.op_code() {
                        OpCode::And => self.and(graph, seen, arg)?,
                        OpCode::Or => self.or(graph, seen, arg)?,
                        OpCode::Xor => self.xor(graph, seen, arg)?,
                        _ => return Err(ESError::corrupted_rpn_stack()),
                    };
                    if operator.negated() {
                        result = !result;
                    }
                    stack.push(Token::Solved(result));
                    return Ok(());
                } else {
                    self.dual_token(graph, seen, (arg.1, third), stack)?;
                    stack.push(arg.0);
                    return Ok(());
                }
            }
        }
    }

    fn and(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        arg: (Token, Token),
    ) -> ESResult<bool> {
        let left = self.single_token(graph, seen, arg.0)?;
        let right = self.single_token(graph, seen, arg.1)?;

        Ok(left && right)
    }

    fn or(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        arg: (Token, Token),
    ) -> ESResult<bool> {
        let left = self.single_token(graph, seen, arg.0)?;
        let right = self.single_token(graph, seen, arg.1)?;

        Ok(left || right)
    }

    fn xor(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
        arg: (Token, Token),
    ) -> ESResult<bool> {
        let left = self.single_token(graph, seen, arg.0)?;
        let right = self.single_token(graph, seen, arg.1)?;

        Ok((left && !right) || (!left && right))
    }
}
