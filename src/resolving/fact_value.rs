/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fact_value.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/12/04 11:13:55 by gsmith            #+#    #+#             */
/*   Updated: 2019/12/04 12:56:21 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, Copy)]
pub enum FactValue {
    Undefined,
    Fixed(bool),
    Absurd,
}

impl Default for FactValue {
    fn default() -> Self {
        FactValue::Undefined
    }
}
