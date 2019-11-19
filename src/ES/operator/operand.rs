/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 14:50:58 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 12:46:10 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{super::Graph, Operator};

pub struct Operand {
    symbol: char,
}

impl Operand {
    pub fn new(symbol: char) -> Self {
        Operand { symbol }
    }
}

impl Operator for Operand {
    fn process(&self, graph: &Graph) -> bool {
        graph.solve_fact(self.symbol)
    }
}

#[cfg(test)]
mod process {
    use super::{super::Graph, Operand, Operator};

    #[test]
    fn from_true() {
        let mut graph = Graph::new();
        let fact = 'A';
        graph.create_fact(fact);
        if let Err(_) = graph.set_fact(fact, true) {
            assert!(false, "Error while setting up fact");
        }

        let operand_a = Operand::new(fact);
        assert!(operand_a.process(&graph), "Operand A wasn't true");
    }

    #[test]
    fn from_false() {
        let mut graph = Graph::new();
        let fact = 'B';
        graph.create_fact(fact);
        if let Err(_) = graph.set_fact(fact, false) {
            assert!(false, "Error while setting up fact");
        }

        let operand_b = Operand::new(fact);
        assert!(!operand_b.process(&graph), "Operand B wasn't false");
    }

    #[test]
    fn from_unexisting() {
        let mut graph = Graph::new();
        let fact = 'C';
        graph.create_fact(fact);

        let operand_c = Operand::new(fact);
        assert!(!operand_c.process(&graph), "Operand C wasn't false");
    }
}
