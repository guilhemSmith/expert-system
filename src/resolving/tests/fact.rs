/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fact.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/22 17:20:39 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/03 17:12:54 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::super::Graph;
use crate::utils::error::ESResult;

#[test]
fn solve_true() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('A');
    graph.init_fact('A')?;

    assert_eq!(
        graph.solve_fact('A')?,
        Some(true),
        "Expected 'A' to be true, but it isn't."
    );
    Ok(())
}

#[test]
fn solve_false() -> ESResult<()> {
    let mut graph = Graph::new();
    graph.create_fact('B');

    assert_eq!(
        graph.solve_fact('B')?,
        Some(false),
        "Expected 'B' to be false, but it isn't."
    );
    Ok(())
}
