/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   state_process.rs                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/29 16:45:12 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/03 16:41:27 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashSet;

use crate::lexer_parser::{ESLine, ESLineType};
use crate::resolving::Graph;
use crate::utils::{
    error::{ESErrorKind, ESResult},
    token::{ModifierType, Token},
};

pub fn run(filename: String) -> ESResult<()> {
    let mut graph = Graph::new();
    let mut start_solving = false;

    let lines = ESLine::process(&filename)?;
    for line in lines {
        line.check()?;
        match line.kind() {
            ESLineType::Empty => continue,
            ESLineType::Rule => {
                if start_solving {
                    graph.clear_solutions();
                    start_solving = false;
                }
                process_rules(&mut graph, line)?
            }
            ESLineType::Fact => {
                if start_solving {
                    graph.clear_solutions();
                    start_solving = false;
                }
                process_fact(&mut graph, line)?
            }
            ESLineType::Query => {
                start_solving = true;
                process_query(&mut graph, line)?
            }
        };
    }
    Ok(())
}

fn process_rules(graph: &mut Graph, line: ESLine) -> ESResult<()> {
    let mut left_set: HashSet<char> = HashSet::new();
    let mut right_set: HashSet<char> = HashSet::new();
    let mut current_set = &mut left_set;

    for tok in line.tokens() {
        match tok {
            Token::Factual(op) => {
                current_set.insert(op.symbol());
            }
            Token::Behavioral(modif) => {
                if let ModifierType::Imply = modif.symbol() {
                    current_set = &mut right_set;
                };
            }
            _ => continue,
        }
    }
    for symbol in &left_set {
        graph.create_fact(*symbol);
    }
    for symbol in &right_set {
        graph.create_fact(*symbol);
    }
    let rpn = line.to_prefix()?;
    graph.add_rule(rpn, left_set, right_set)?;
    Ok(())
}

fn process_fact(graph: &mut Graph, line: ESLine) -> ESResult<()> {
    for tok in line.tokens() {
        match tok {
            Token::Factual(op) => {
                if let Err(err) = graph.init_fact(op.symbol()) {
                    match err.kind() {
                        ESErrorKind::UnknownFact => {
                            graph.create_fact(op.symbol());
                            graph.init_fact(op.symbol())?;
                        }
                        _ => return Err(err),
                    }
                };
            }
            _ => continue,
        }
    }
    Ok(())
}

fn process_query(graph: &mut Graph, line: ESLine) -> ESResult<()> {
    for tok in line.tokens() {
        match tok {
            Token::Factual(op) => {
                let val = graph.solve_fact(op.symbol())?;
                println!(
                    "{}={}",
                    op.symbol(),
                    match val {
                        None => "absurd",
                        Some(val) =>
                            if val {
                                "true"
                            } else {
                                "false"
                            },
                    }
                );
            }
            _ => continue,
        };
    }
    Ok(())
}
