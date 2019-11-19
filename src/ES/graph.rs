/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   graph.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:06:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/19 16:41:52 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use std::result::Result;
use super::{ESError, ESErrorKind, Fact, Rule};

pub struct Graph<'r> {
    facts: HashMap<char, Fact<'r>>,
    rules: Vec<Rule>,
}

impl<'r> Graph<'r> {
    pub fn new() -> Self {
        Graph {
            facts: HashMap::new(),
            rules: Vec::new(),
        }
    }

    pub fn create_fact(&mut self, symbol: char) {
        if let None = self.facts.get(&symbol) {
            self.facts.insert(symbol, Fact::new(None));
        }
    }

    pub fn set_fact(&mut self, symbol: char, val: bool) -> Result<(), ESError> {
        if let Some(fact) = self.facts.get_mut(&symbol) {
            fact.set_val(val);
            return Ok(());
        } else {
            return Err(ESError::new(ESErrorKind::UnknownFact));
        }
    }

    pub fn solve(&self, symbols: HashSet<char>) {
        for symbol in symbols {
            println!(
                "{} = {}",
                symbol,
                if self.solve_fact(symbol) {
                    "true"
                } else {
                    "false"
                }
            )
        }
    }

    pub fn solve_fact(&self, symbol: char) -> bool {
        match self.facts.get(&symbol) {
            None => false,
            Some(fact) => fact.solve(self),
        }
    }
}
