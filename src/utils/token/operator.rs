/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/20 10:47:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/21 17:46:43 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Copy, Clone)]
pub enum OpCode {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
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

    pub fn op_code(&self) -> OpCode {
        self.opc
    }
}
