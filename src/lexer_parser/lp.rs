/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lp.rs                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: fle-roy <francis.leroy@protonmail.ch>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/11/14 15:41:50 by fle-roy           #+#    #+#             */
/*   Updated: 2019/11/14 18:19:20 by fle-roy          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::errors::ESErrors::ESError;

enum LineType {
	Empty,
    Rule,
	Fact,
	Query,
}

fn get_line(line: &mut String) -> Result<(), ESError>
{
	// Get the line without comment
	Ok(())
}

fn get_line_type(line: &String) -> Result<LineType, ESError>
{
	//Get line type
	Err(ESError::unknown_input_line_type())
}

pub fn process_file(path: &String) -> Result<(), ESError>
{
	let file = std::fs::File::open(path)?;
	Ok(())
}
