/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 14:48:22 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/18 12:39:02 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod and;
mod not;
mod operand;
mod or;
mod xor;

pub use and::And;
pub use not::Not;
pub use operand::Operand;
pub use or::Or;
pub use xor::Xor;

use super::Graph;

pub trait Operator {
    fn process(&self, graph: &Graph) -> bool;
}
