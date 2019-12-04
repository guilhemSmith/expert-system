/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   modifier.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/20 10:50:03 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/27 18:52:51 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::utils::error::{ESError, ESErrorKind};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModifierType {
    Deind,
    Imply,
    Ind,
}

#[derive(Copy, Clone, Debug)]
pub struct Modifier {
    negated: bool,
    symbol: ModifierType,
}

impl PartialEq for Modifier {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.negated == other.negated
    }
}

impl Modifier {
    pub fn new(negated: bool, symbol: ModifierType) -> Result<Self, ESError> {
        match symbol {
            ModifierType::Ind => Ok(Modifier { negated, symbol }),
            ModifierType::Deind | ModifierType::Imply => {
                if negated {
                    Err(ESError::new_w_what(
                        ESErrorKind::LineError,
                        format!(
                            "Can't negate the impy `{}` operator",
                            Modifier::op_to_str(symbol)
                        ),
                    ))
                } else {
                    Ok(Modifier { negated, symbol })
                }
            }
        }
    }

    pub fn ind(&self) -> i8 {
        match self.symbol() {
            ModifierType::Deind => -1,
            ModifierType::Imply => 0,
            ModifierType::Ind => 1,
        }
    }

    pub fn is_ind(&self) -> bool {
        match self.symbol {
            ModifierType::Deind | ModifierType::Ind => true,
            ModifierType::Imply => false,
        }
    }

    pub fn negated(&self) -> bool {
        self.negated
    }

    pub fn symbol(&self) -> ModifierType {
        self.symbol
    }

    pub fn op_to_str(symbol: ModifierType) -> String {
        match symbol {
            ModifierType::Ind => format!("{}", '('),
            ModifierType::Imply => format!("{}", "=>"),
            ModifierType::Deind => format!("{}", ')'),
        }
    }

    pub fn op_from_str(symbol: &String) -> Result<ModifierType, ESError> {
        match &symbol[..] {
            "(" => Ok(ModifierType::Ind),
            ")" => Ok(ModifierType::Deind),
            "=>" => Ok(ModifierType::Imply),
            _ => Err(ESError::new(ESErrorKind::UnknownOp)),
        }
    }

    pub fn display_str(&self) -> String {
        format!("{}", Modifier::op_to_str(self.symbol()))
    }
}
