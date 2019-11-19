/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rule.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:07:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 13:22:35 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{operator::Operator, Graph};

pub struct Rule {
    condition: Box<dyn Operator>,
}

impl Rule {
    pub fn new(condition: Box<dyn Operator>) -> Self {
        Rule { condition }
    }

    pub fn process(&self, graph: &Graph) -> bool {
        self.condition.process(graph)
    }
}
