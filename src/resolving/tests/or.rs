/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   or.rs                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/22 17:21:27 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/22 17:21:36 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashSet;

use super::super::Graph;
use crate::utils::{
    error::ESResult,
    token::{OpCode, Operand, Operator, Token},
};

fn build() -> ESResult<Graph> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    let condition = vec![
        Token::Computable(Operator::new(false, OpCode::Or)),
        Token::Factual(Operand::new(false, 'B')),
        Token::Factual(Operand::new(false, 'A')),
    ];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    in_facts.insert('A');
    in_facts.insert('B');
    out_facts.insert('C');
    graph.add_rule(condition, in_facts, out_facts)?;
    return Ok(graph);
}

#[test]
fn solve_true_false() -> ESResult<()> {
    let mut graph = build()?;
    graph.init_fact('A')?;

    assert!(
        graph.solve_fact('C')?,
        "Expected 'C' to be true, but it isn't."
    );
    Ok(())
}

#[test]
fn solve_true_only() -> ESResult<()> {
    let mut graph = build()?;
    graph.init_fact('A')?;
    graph.init_fact('B')?;

    assert!(
        graph.solve_fact('C')?,
        "Expected 'C' to be true, but it isn't."
    );
    Ok(())
}

#[test]
fn solve_false_only() -> ESResult<()> {
    let mut graph = build()?;

    assert!(
        !graph.solve_fact('C')?,
        "Expected 'C' to be false, but it isn't."
    );
    Ok(())
}
