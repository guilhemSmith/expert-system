/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   graph.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:06:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/22 17:02:17 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use super::{Fact, Rule};
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
        seen: &mut HashMap<char, Option<bool>>,
    ) -> ESResult<bool> {
        let res = match self.facts.get(&symbol) {
            None => false,
            Some(fact) => fact.solve(self, seen)?,
        };
        return Ok(res);
    }

    pub fn try_rule(
        &self,
        id: usize,
        seen: &mut HashMap<char, Option<bool>>,
    ) -> ESResult<bool> {
        self.rules[id].process(self, seen)
    }

    pub fn solve_fact(&mut self, symbol: char) -> ESResult<bool> {
        let mut seen: HashMap<char, Option<bool>> = HashMap::new();
        let res = self.try_fact(symbol, &mut seen)?;
        for (key, value) in seen {
            if let Some(res) = value {
                match self.facts.get_mut(&key) {
                    None => {}
                    Some(fact) => fact.set_solved_value(res),
                }
            }
        }
        return Ok(res);
    }
}
