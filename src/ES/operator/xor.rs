/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   xor.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/18 11:46:37 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 12:46:32 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use super::{super::Graph, Operator};

pub struct Xor {
    left: Box<dyn Operator>,
    right: Box<dyn Operator>,
}

impl Xor {
    pub fn new(left: Box<dyn Operator>, right: Box<dyn Operator>) -> Self {
        Xor { left, right }
    }
}

impl Operator for Xor {
    fn process(&self, graph: &Graph) -> bool {
        (!self.left.process(graph) && self.right.process(graph))
            || (self.left.process(graph) && !self.right.process(graph))
    }
}

#[cfg(test)]
mod process {
    use super::{
        super::{Graph, Operand},
        Operator, Xor,
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

        let xor = Xor::new(Box::new(left), Box::new(right));
        assert!(xor.process(&graph), "'xor' wasn't true with left true");
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

        let xor = Xor::new(Box::new(left), Box::new(right));
        assert!(xor.process(&graph), "xor' wasn't true with right true");
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

        let xor = Xor::new(Box::new(left), Box::new(right));
        assert!(!xor.process(&graph), "'xor' wasn't false with both true");
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

        let xor = Xor::new(Box::new(left), Box::new(right));
        assert!(!xor.process(&graph), "'xor' wasn't false with both false");
    }
}
