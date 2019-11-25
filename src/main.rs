/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/15 11:09:23 by gsmith            #+#    #+#             */
/*   Updated: 2019/11/19 18:07:41 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod lexer_parser;
pub mod resolving;
pub mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    lexer_parser::process_file(&args[0]);
}
