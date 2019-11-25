/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/20 10:47:08 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/21 17:46:06 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod operand;
mod operator;

pub use operand::Operand;
pub use operator::{OpCode, Operator};

#[derive(Clone)]
pub enum Token {
    Computable(Operator),
    Factual(Operand),
    Solved(bool),
}
