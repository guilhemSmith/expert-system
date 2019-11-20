/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/20 10:47:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/20 11:34:05 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub enum OpCode {
    And,
    Or,
    Xor,
}

pub struct Operator {
    negated: bool,
    opc: OpCode,
}

impl Operator {
    pub fn new(negated: bool, opc: OpCode) -> Self {
        Operator { negated, opc }
    }

    pub fn is_negated(&self) -> bool {
        self.negated
    }
}
