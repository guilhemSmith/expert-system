/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fact.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:07:14 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/03 10:47:02 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashMap;
use std::vec::Vec;

use super::Graph;
use crate::utils::error::ESResult;

pub struct Fact {
    init_val: bool,
    solved: Option<bool>,
    rules_id: Vec<usize>,
}

impl Fact {
    pub fn new() -> Self {
        Fact {
            init_val: false,
            solved: None,
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
        self.solved = Some(value);
    }

    pub fn solve(
        &self,
        graph: &Graph,
        seen: &mut HashMap<char, Option<bool>>,
    ) -> ESResult<bool> {
        if self.init_val {
            return Ok(true);
        } else {
            match self.solved {
                Some(res) => return Ok(res),
                None => {
                    for id in &self.rules_id {
                        if graph.try_rule(*id, seen)? {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }
            }
        }
    }

    pub fn clear_solution(&mut self) {
        self.solved = None;
    }
}
