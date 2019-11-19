/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   not.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/18 12:38:37 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 12:50:37 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{super::Graph, Operator};

pub struct Not {
	negated: Box<dyn Operator>,
}

impl Not {
    pub fn new(negated: Box<dyn Operator>) -> Self {
        Not { negated }
    }
}

impl Operator for Not {
    fn process(&self, graph: &Graph) -> bool {
        !self.negated.process(graph)
    }
}

#[cfg(test)]
mod process {
    use super::{
        super::{Graph, Operand},
        Operator, Not,
    };

    #[test]
    fn from_true() {
        let mut graph = Graph::new();
        let fact = 'A';
        graph.create_fact(fact);
        if let Err(_) = graph.set_fact(fact, true) {
            assert!(false, "Error while setting up fact right");
        }
        let operand = Operand::new(fact);

        let not = Not::new(Box::new(operand));
        assert!(!not.process(&graph), "'not' wasn't false from true");
    }

    #[test]
    fn from_false() {
        let mut graph = Graph::new();
        let fact = 'B';
        graph.create_fact(fact);
        if let Err(_) = graph.set_fact(fact, false) {
            assert!(false, "Error while setting up fact right");
        }
        let operand = Operand::new(fact);

        let not = Not::new(Box::new(operand));
        assert!(not.process(&graph), "'not' wasn't true from false");
    }
}