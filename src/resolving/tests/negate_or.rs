/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   negate_or.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/22 17:25:44 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/22 17:26:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashSet;

use super::super::Graph;
use crate::utils::{
    error::ESResult,
    token::{OpCode, Operand, Operator, Token},
};

#[test]
fn solve_operator_true_negated() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    let condition = vec![
        Token::Computable(Operator::new(true, OpCode::Or)),
        Token::Factual(Operand::new(false, 'B')),
        Token::Factual(Operand::new(false, 'A')),
    ];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    in_facts.insert('A');
    in_facts.insert('B');
    out_facts.insert('C');
    graph.add_rule(condition, in_facts, out_facts)?;
    graph.init_fact('A')?;

    assert!(
        !graph.solve_fact('C')?,
        "Expected 'C' to be false, but it isn't."
    );
    Ok(())
}

#[test]
fn solve_operator_false_negated() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    let condition = vec![
        Token::Computable(Operator::new(true, OpCode::Or)),
        Token::Factual(Operand::new(false, 'B')),
        Token::Factual(Operand::new(false, 'A')),
    ];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    in_facts.insert('A');
    in_facts.insert('B');
    out_facts.insert('C');
    graph.add_rule(condition, in_facts, out_facts)?;

    assert!(
        graph.solve_fact('C')?,
        "Expected 'C' to be true, but it isn't."
    );
    Ok(())
}

#[test]
fn solve_operands_true_negated() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    let condition = vec![
        Token::Computable(Operator::new(false, OpCode::Or)),
        Token::Factual(Operand::new(false, 'B')),
        Token::Factual(Operand::new(true, 'A')),
    ];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    in_facts.insert('A');
    in_facts.insert('B');
    out_facts.insert('C');
    graph.add_rule(condition, in_facts, out_facts)?;
    graph.init_fact('A')?;

    assert!(
        !graph.solve_fact('C')?,
        "Expected 'C' to be false, but it isn't."
    );
    Ok(())
}

#[test]
fn solve_operands_false_negated() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.create_fact('B');
    graph.create_fact('C');
    let condition = vec![
        Token::Computable(Operator::new(false, OpCode::Or)),
        Token::Factual(Operand::new(false, 'B')),
        Token::Factual(Operand::new(true, 'A')),
    ];
    let mut in_facts = HashSet::new();
    let mut out_facts = HashSet::new();
    in_facts.insert('A');
    in_facts.insert('B');
    out_facts.insert('C');
    graph.add_rule(condition, in_facts, out_facts)?;

    assert!(
        graph.solve_fact('C')?,
        "Expected 'C' to be true, but it isn't."
    );
    Ok(())
}
