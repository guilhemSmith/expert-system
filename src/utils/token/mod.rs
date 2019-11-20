/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/20 10:47:08 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/20 10:58:33 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod operand;
mod operator;

pub use operand::Operand;
pub use operator::Operator;

pub enum Token {
    Computable(Operator),
    Factual(Operand),
}
