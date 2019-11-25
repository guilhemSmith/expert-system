/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/20 10:50:03 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/21 17:46:18 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Clone)]
pub struct Operand {
    negated: bool,
    symbol: char,
}

impl Operand {
    pub fn new(negated: bool, symbol: char) -> Self {
        Operand { negated, symbol }
    }

    pub fn is_negated(&self) -> bool {
        self.negated
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }
}
