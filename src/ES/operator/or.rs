/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   or.rs                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 15:01:57 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 12:46:59 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{super::Graph, Operator};

pub struct Or {
    left: Box<dyn Operator>,
    right: Box<dyn Operator>,
}

impl Or {
    pub fn new(left: Box<dyn Operator>, right: Box<dyn Operator>) -> Self {
        Or { left, right }
    }
}

impl Operator for Or {
    fn process(&self, graph: &Graph) -> bool {
        self.left.process(graph) || self.right.process(graph)
    }
}

#[cfg(test)]
mod process {
    use super::{
        super::{Graph, Operand},
        Operator, Or,
    };

    #[test]
    fn with_left_true() {
        let mut graph = Graph::new();
        let fact_left = 'A';
        let fact_right = 'B';
        graph.create_fact(fact_left);
        graph.create_fact(fact_right);
        if let Err(_) = graph.set_fact(fact_left, true) {
            assert!(false, "Error while setting up fact left");
        }
        if let Err(_) = graph.set_fact(fact_right, false) {
            assert!(false, "Error while setting up fact right");
        }
        let left = Operand::new(fact_left);
        let right = Operand::new(fact_right);

        let or = Or::new(Box::new(left), Box::new(right));
        assert!(or.process(&graph), "'or' wasn't true with left true");
    }

    #[test]
    fn with_right_true() {
        let mut graph = Graph::new();
        let fact_left = 'A';
        let fact_right = 'B';
        graph.create_fact(fact_left);
        graph.create_fact(fact_right);
        if let Err(_) = graph.set_fact(fact_left, false) {
            assert!(false, "Error while setting up fact left");
        }
        if let Err(_) = graph.set_fact(fact_right, true) {
            assert!(false, "Error while setting up fact right");
        }
        let left = Operand::new(fact_left);
        let right = Operand::new(fact_right);

        let or = Or::new(Box::new(left), Box::new(right));
        assert!(or.process(&graph), "or' wasn't true with right true");
    }

    #[test]
    fn with_both_true() {
        let mut graph = Graph::new();
        let fact_left = 'A';
        let fact_right = 'B';
        graph.create_fact(fact_left);
        graph.create_fact(fact_right);
        if let Err(_) = graph.set_fact(fact_left, true) {
            assert!(false, "Error while setting up fact left");
        }
        if let Err(_) = graph.set_fact(fact_right, true) {
            assert!(false, "Error while setting up fact right");
        }
        let left = Operand::new(fact_left);
        let right = Operand::new(fact_right);

        let or = Or::new(Box::new(left), Box::new(right));
        assert!(or.process(&graph), "'or' wasn't true with both true");
    }

    #[test]
    fn with_both_false() {
        let mut graph = Graph::new();
        let fact_left = 'A';
        let fact_right = 'B';
        graph.create_fact(fact_left);
        graph.create_fact(fact_right);
        if let Err(_) = graph.set_fact(fact_left, false) {
            assert!(false, "Error while setting up fact left");
        }
        if let Err(_) = graph.set_fact(fact_right, false) {
            assert!(false, "Error while setting up fact right");
        }
        let left = Operand::new(fact_left);
        let right = Operand::new(fact_right);

        let or = Or::new(Box::new(left), Box::new(right));
        assert!(!or.process(&graph), "'or' wasn't false with both false");
    }
}
