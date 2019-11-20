/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   Error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:46:59 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/19 16:54:10 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::error::Error;
use std::fmt;

pub enum ESErrorKind {
    UnknownFact,
    IOError,
}

pub struct ESError {
    kind: ESErrorKind,
    what: Option<String>,
    recov: bool,
}

impl ESError {
    pub fn new(kind: ESErrorKind) -> ESError {
        ESError {
            kind: kind,
            what: None,
            recov: true,
        }
    }

    pub fn kind(&self) -> String {
        match self.kind {
            ESErrorKind::UnknownFact => String::from("Logic Error"),
            ESErrorKind::IOError => String::from("IO Error"),
        }
    }

    pub fn what(&self) -> Option<String> {
        match self.kind {
            ESErrorKind::UnknownFact => Some(String::from("Unknown fact")),
            _ => self.what.clone(),
        }
    }
}

impl fmt::Display for ESError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.kind(),
            match self.what() {
                None => String::from("unexpected"),
                Some(msg) => msg,
            }
        )
    }
}

impl fmt::Debug for ESError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{}:{}> {}: {}",
            file!(),
            line!(),
            self.kind(),
            match self.what() {
                None => String::from("unexpected"),
                Some(msg) => msg,
            }
        )
    }
}
impl Error for ESError {}

impl From<std::io::Error> for ESError {
    fn from(error: std::io::Error) -> Self {
        ESError {
            kind: ESErrorKind::IOError,
            what: Some(String::from(error.description())),
            recov: false,
        }
    }
}
