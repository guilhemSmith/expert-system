/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   graph.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:06:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/04 13:21:00 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use super::{Fact, FactValue, Rule};
use crate::utils::{
    error::{ESError, ESResult},
    token::Token,
};

pub struct Graph {
    facts: HashMap<char, Fact>,
    rules: Vec<Rule>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            facts: HashMap::new(),
            rules: Vec::new(),
        }
    }

    pub fn create_fact(&mut self, symbol: char) {
        if let None = self.facts.get(&symbol) {
            self.facts.insert(symbol, Fact::new());
        }
    }

    pub fn init_fact(&mut self, symbol: char) -> ESResult<()> {
        if let Some(fact) = self.facts.get_mut(&symbol) {
            fact.set_init_val();
            return Ok(());
        } else {
            return Err(ESError::unknown_fact(symbol));
        }
    }

    pub fn add_rule(
        &mut self,
        condition: Vec<Token>,
        in_facts_id: HashSet<char>,
        out_facts_id: HashSet<char>,
    ) -> ESResult<()> {
        let rule = Rule::new(condition, in_facts_id);
        let rule_id = self.rules.len();
        for symbol in out_facts_id {
            self.facts.get_mut(&symbol).unwrap().add_rule_id(rule_id);
        }
        self.rules.push(rule);
        Ok(())
    }

    pub fn fact_ref(&self, symbol: char) -> ESResult<&Fact> {
        match self.facts.get(&symbol) {
            None => Err(ESError::unknown_fact(symbol)),
            Some(fact) => Ok(fact),
        }
    }

    pub fn try_fact(
        &self,
        symbol: char,
        seen: &mut HashMap<char, FactValue>,
    ) -> ESResult<FactValue> {
        let res = match self.facts.get(&symbol) {
            None => FactValue::Undefined,
            Some(fact) => fact.solve(self, seen)?,
        };
        return Ok(res);
    }

    pub fn try_rule(
        &self,
        id: usize,
        seen: &mut HashMap<char, FactValue>,
    ) -> ESResult<FactValue> {
        self.rules[id].process(self, seen)
    }

    pub fn solve_fact(&mut self, symbol: char) -> ESResult<Option<bool>> {
        let mut seen: HashMap<char, FactValue> = HashMap::new();
        let res = self.try_fact(symbol, &mut seen)?;
        for (key, value) in seen {
            match value {
                FactValue::Fixed(val) => match self.facts.get_mut(&key) {
                    None => continue,
                    Some(fact) => fact.set_solved_value(val),
                },
                _ => continue,
            }
        }
        match res {
            FactValue::Fixed(val) => match self.facts.get_mut(&symbol) {
                None => return Ok(Some(val)),
                Some(fact) => {
                    fact.set_solved_value(val);
                    return Ok(Some(val));
                }
            },
            FactValue::Undefined => return Ok(Some(false)),
            FactValue::Absurd => return Ok(None),
        }
    }

    pub fn clear_solutions(&mut self) {
        for (_, fact) in &mut self.facts {
            fact.clear_solution();
        }
    }
}
