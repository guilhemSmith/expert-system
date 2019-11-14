/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ESErrors.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/14 16:37:03 by fle-roy           #+#    #+#             */
/*   Updated: 2019/11/14 18:19:22 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum ESErrorKind {
	Unknown,
	UnknownInput,
	UnknownInputLineType,
}

impl fmt::Display for ESErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ESErrorKind::Unknown => write!(f, "parser"),
            ESErrorKind::UnknownInput => write!(f, "parser"),
            ESErrorKind::UnknownInputLineType => write!(f, "parser"),
        }
    }
}


#[derive(Debug, Clone)]
pub struct ESError {
    kind: ESErrorKind,
    info: String,
}

impl ESError {
    pub fn kind(&self) -> &ESErrorKind {
        &self.kind
    }

    pub fn unknown() -> Self {
        ESError {
            kind: ESErrorKind::Unknown,
            info: format!("Unkown error"),
        }
	}
	
	pub fn unknown_input() -> Self {
        ESError {
            kind: ESErrorKind::UnknownInput,
            info: format!("Unkown error while parsing input"),
        }
	}
	
	pub fn unknown_input_line_type() -> Self {
        ESError {
            kind: ESErrorKind::UnknownInputLineType,
            info: format!("Unkown line type while parsing input"),
        }
    }
}

impl Error for ESError {}

impl fmt::Display for ESError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            _ => write!(f, "{} - {}", self.kind, self.info),
        }
    }
}
