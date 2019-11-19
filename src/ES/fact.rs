/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fact.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:07:14 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/15 14:47:46 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::vec::Vec;

use super::{Graph, Rule};

pub struct Fact<'r> {
    init_val: Option<bool>,
    rules: Vec<&'r Rule>,
}

impl<'r> Fact<'r> {
    pub fn new(init_val: Option<bool>) -> Self {
        Fact {
            init_val,
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: &'r Rule) {
        self.rules.push(rule);
    }

    pub fn set_val(&mut self, val: bool) {
        self.init_val = Some(val);
    }

    pub fn solve(&self, graph: &Graph) -> bool {
        match self.init_val {
            Some(val) => return val,
            None => {
                for rule in &self.rules {
                    if rule.process(graph) {
                        return true;
                    }
                }
                return false;
            }
        }
    }
}
