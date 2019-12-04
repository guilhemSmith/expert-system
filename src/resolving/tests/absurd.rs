/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   absurd.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/12/04 10:36:03 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/04 10:57:08 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashSet;

use super::super::Graph;
use crate::utils::{
    error::ESResult,
    token::{Operand, Token},
};

fn add_chain(
    graph: &mut Graph,
    start: char,
    next: char,
    neg: bool,
) -> ESResult<()> {
    let condition = vec![Token::Factual(Operand::new(neg, start))];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    out_facts.insert(next);
    in_facts.insert(start);
    graph.add_rule(condition, in_facts, out_facts)?;
    Ok(())
}

#[test]
fn negated_circle_short_00() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    add_chain(&mut graph, 'B', 'A', false)?;
    add_chain(&mut graph, 'A', 'B', true)?;
    assert_eq!(
        graph.solve_fact('A')?,
        None,
        "Expected 'A' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('B')?,
        None,
        "Expected 'B' to be absurd, but it isn't."
    );
    Ok(())
}

#[test]
fn negated_circle_short_01() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    add_chain(&mut graph, 'B', 'A', false)?;
    add_chain(&mut graph, 'A', 'B', true)?;
    assert_eq!(
        graph.solve_fact('B')?,
        None,
        "Expected 'B' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('A')?,
        None,
        "Expected 'A' to be absurd, but it isn't."
    );
    Ok(())
}

#[test]
fn negated_circle_long_00() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    add_chain(&mut graph, 'A', 'B', false)?;
    add_chain(&mut graph, 'B', 'C', true)?;
    add_chain(&mut graph, 'C', 'D', false)?;
    add_chain(&mut graph, 'D', 'A', false)?;
    assert_eq!(
        graph.solve_fact('A')?,
        None,
        "Expected 'A' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('B')?,
        None,
        "Expected 'B' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('C')?,
        None,
        "Expected 'C' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('D')?,
        None,
        "Expected 'D' to be absurd, but it isn't."
    );
    Ok(())
}

#[test]
fn negated_circle_long_01() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    graph.create_fact('D');
    add_chain(&mut graph, 'A', 'B', false)?;
    add_chain(&mut graph, 'B', 'C', true)?;
    add_chain(&mut graph, 'C', 'D', false)?;
    add_chain(&mut graph, 'D', 'A', false)?;
    assert_eq!(
        graph.solve_fact('D')?,
        None,
        "Expected 'D' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('C')?,
        None,
        "Expected 'C' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('B')?,
        None,
        "Expected 'B' to be absurd, but it isn't."
    );
    assert_eq!(
        graph.solve_fact('A')?,
        None,
        "Expected 'A' to be absurd, but it isn't."
    );
    Ok(())
}
