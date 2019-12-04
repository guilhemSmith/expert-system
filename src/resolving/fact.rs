/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fact.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:07:14 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/04 13:48:15 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashMap;
use std::vec::Vec;

use super::{FactValue, Graph};
use crate::utils::error::ESResult;

pub struct Fact {
    init_val: bool,
    solved: FactValue,
    rules_id: Vec<usize>,
}

impl Fact {
    pub fn new() -> Self {
        Fact {
            init_val: false,
            solved: FactValue::default(),
            rules_id: Vec::new(),
        }
    }

    pub fn add_rule_id(&mut self, id: usize) {
        self.rules_id.push(id);
    }

    pub fn set_init_val(&mut self) {
        self.init_val = true;
    }

    pub fn set_solved_value(&mut self, value: bool) {
        self.solved = FactValue::Fixed(value);
    }

    pub fn solve(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, FactValue>,
    ) -> ESResult<FactValue> {
        if self.init_val {
            return Ok(FactValue::Fixed(true));
        } else {
            match self.solved {
                FactValue::Undefined => {
                    if self.rules_id.len() == 0 {
                        return Ok(FactValue::Fixed(false));
                    }
                    for id in &self.rules_id {
                        match graph.try_rule(*id, seen)? {
                            FactValue::Absurd => return Ok(FactValue::Absurd),
                            FactValue::Fixed(true) => {
                                return Ok(FactValue::Fixed(true))
                            }
                            _ => continue,
                        }
                    }
                    return Ok(FactValue::Undefined);
                }
                _ => return Ok(self.solved),
            }
        }
    }

    pub fn clear_solution(&mut self) {
        self.solved = FactValue::Undefined;
    }
}
