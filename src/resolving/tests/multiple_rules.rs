/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   multiple_rules.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/22 17:38:07 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/22 17:55:44 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashSet;

use super::super::Graph;
use crate::utils::{
    error::ESResult,
    token::{Operand, Token},
};

fn build_chain(graph: &mut Graph, from: char, to: char) -> ESResult<()> {
    let condition = vec![Token::Factual(Operand::new(false, from))];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    in_facts.insert(from);
    out_facts.insert(to);
    graph.add_rule(condition, in_facts, out_facts)?;
    Ok(())
}

#[test]
fn chain_from_true() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    graph.create_fact('E');
    build_chain(&mut graph, 'A', 'B')?;
    build_chain(&mut graph, 'B', 'C')?;
    build_chain(&mut graph, 'C', 'D')?;
    build_chain(&mut graph, 'D', 'E')?;
    graph.init_fact('A')?;

    assert!(
        graph.solve_fact('E')?,
        "Expected 'E' to be true, but it isn't."
    );
    Ok(())
}

#[test]
fn chain_from_false() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    graph.create_fact('E');
    build_chain(&mut graph, 'A', 'B')?;
    build_chain(&mut graph, 'B', 'C')?;
    build_chain(&mut graph, 'C', 'D')?;
    build_chain(&mut graph, 'D', 'E')?;

    assert!(
        !graph.solve_fact('E')?,
        "Expected 'E' to be false, but it isn't."
    );
    Ok(())
}

#[test]
fn circle_from_true() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    graph.create_fact('E');
    build_chain(&mut graph, 'A', 'B')?;
    build_chain(&mut graph, 'B', 'C')?;
    build_chain(&mut graph, 'C', 'D')?;
    build_chain(&mut graph, 'D', 'E')?;
    build_chain(&mut graph, 'E', 'A')?;
    graph.init_fact('A')?;

    assert!(
        graph.solve_fact('E')?,
        "Expected 'E' to be true, but it isn't."
    );
    Ok(())
}

#[test]
fn circle_from_false() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    graph.create_fact('E');
    build_chain(&mut graph, 'A', 'B')?;
    build_chain(&mut graph, 'B', 'C')?;
    build_chain(&mut graph, 'C', 'D')?;
    build_chain(&mut graph, 'D', 'E')?;
    build_chain(&mut graph, 'E', 'A')?;

    assert!(
        !graph.solve_fact('E')?,
        "Expected 'E' to be false, but it isn't."
    );
    Ok(())
}

#[test]
fn four_false() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    graph.create_fact('E');
    build_chain(&mut graph, 'A', 'E')?;
    build_chain(&mut graph, 'B', 'E')?;
    build_chain(&mut graph, 'C', 'E')?;
    build_chain(&mut graph, 'D', 'E')?;

    assert!(
        !graph.solve_fact('E')?,
        "Expected 'E' to be false, but it isn't."
    );
    Ok(())
}

#[test]
fn two_false_one_true() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    build_chain(&mut graph, 'A', 'D')?;
    build_chain(&mut graph, 'B', 'D')?;
    build_chain(&mut graph, 'C', 'D')?;
    graph.init_fact('B')?;

    assert!(
        graph.solve_fact('D')?,
        "Expected 'D' to be true, but it isn't."
    );
    Ok(())
}
