/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   and.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/18 11:46:37 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 12:47:14 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use super::{super::Graph, Operator};

pub struct And {
    left: Box<dyn Operator>,
    right: Box<dyn Operator>,
}

impl And {
    pub fn new(left: Box<dyn Operator>, right: Box<dyn Operator>) -> Self {
        And { left, right }
    }
}

impl Operator for And {
    fn process(&self, graph: &Graph) -> bool {
        self.left.process(graph) && self.right.process(graph)
    }
}

#[cfg(test)]
mod process {
    use super::{
        super::{Graph, Operand},
        Operator, And,
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

        let and = And::new(Box::new(left), Box::new(right));
        assert!(!and.process(&graph), "'and' wasn't false with left true");
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

        let and = And::new(Box::new(left), Box::new(right));
        assert!(!and.process(&graph), "and' wasn't false with right true");
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

        let and = And::new(Box::new(left), Box::new(right));
        assert!(and.process(&graph), "'and' wasn't true with both true");
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

        let and = And::new(Box::new(left), Box::new(right));
        assert!(!and.process(&graph), "'and' wasn't false with both false");
    }
}
