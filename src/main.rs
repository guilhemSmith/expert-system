/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/14 15:32:45 by fle-roy           #+#    #+#             */
/*   Updated: 2019/11/14 18:19:20 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod lexer_parser;
pub mod errors;
fn main() {
	let args: Vec<String> = std::env::args().collect();
	
	lexer_parser::lp::process_file(&args[0]);
}
