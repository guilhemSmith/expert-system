use crate::utils::error::{ESError, ESErrorKind, ESResult};
use crate::utils::token::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::*;
#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ESLineType {
    Empty,
    Rule,
    Fact,
    Query,
}

pub struct ESLine {
    line: String,
    line_type: ESLineType,
    tokens: Vec<Token>,
}

impl ESLine {
    fn get_neg(neg: &mut (Vec<bool>, bool)) -> bool {
        if !neg.0.is_empty() {
            return neg.0.last().unwrap() ^ neg.1;
        } else {
            return neg.1;
        }
    }

    fn push_neg(neg: &mut (Vec<bool>, bool)) {
        if !neg.0.is_empty() {
            neg.0.push(neg.0.last().unwrap() ^ neg.1);
        } else {
            neg.0.push(neg.1);
        }
    }

    fn lexer_factual(
        c: &char,
        lt: &Option<ESLineType>,
        res: &mut Vec<Token>,
        neg: &mut (Vec<bool>, bool),
    ) -> ESResult<()> {
        if lt.is_some() && neg.1
        // Query or fact or right side of rule
        {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Can't use negation on the right side of rules"),
            ));
        }
        res.push(Token::Factual(Operand::new(ESLine::get_neg(neg), *c))); // Add to vector
        neg.1 = false; // Don't forget to reset not op
        Ok(())
    }

    fn lexer_indent(
        c: &char,
        res: &mut Vec<Token>,
        neg: &mut (Vec<bool>, bool),
    ) -> ESResult<()> {
        let op = Modifier::op_from_str(&c.to_string())?;

        if neg.1 && op != ModifierType::Ind
        // Neg on `)` or `=>`
        {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Can't use negation modifier with operator {}", c),
            ));
        }
        match op {
            ModifierType::Ind => ESLine::push_neg(neg), // Add to stack
            ModifierType::Deind => {
                neg.0.pop();
                ()
            } // Remove from stack
            _ => (),
        };
        neg.1 = false;
        res.push(Token::Behavioral(Modifier::new(false, op)?)); // Add to vector
        Ok(())
    }

    fn lexer_computable(
        c: &char,
        res: &mut Vec<Token>,
        neg: &mut (Vec<bool>, bool),
    ) -> ESResult<()> {
        if neg.1 {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Can't use negation modifier with modifier {}", c),
            ));
        }
        res.push(Token::Computable(Operator::new_from_char(neg.1, *c)?)); // Add to vector
        neg.1 = false; // Don't forget to reset not op
        Ok(())
    }

    fn lexer_set_line_type(
        lt: &mut Option<ESLineType>,
        neg: &bool,
        t: ESLineType,
    ) -> ESResult<()> {
        if lt.is_some()
        //Already defined, then problem !
        {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Redefinition of line type at char ="),
            ));
        }
        *lt = Some(t);
        if *neg {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Can't use negation modifier before reserved token"),
            ));
        }
        Ok(())
    }

    fn lexer_handle_equal(
        c: &char,
        neg: &bool,
        iter: &mut Peekable<std::str::Chars<'_>>,
        lt: &mut Option<ESLineType>,
        res: &mut Vec<Token>,
    ) -> ESResult<()> {
        let tmp = iter.peek();

        if *neg {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Can't use negation modifier with operator {}", c),
            ));
        }
        match tmp {
            Some('>') => {
                iter.next(); // Discard the `>`
                ESLine::lexer_set_line_type(lt, neg, ESLineType::Rule)?;
                res.push(Token::Behavioral(Modifier::new(
                    *neg,
                    ModifierType::Imply,
                )?))
            }
            Some('A'..='Z') => {
                ESLine::lexer_set_line_type(lt, neg, ESLineType::Fact)?;
                if res.len() > 0 {
                    return Err(ESError::new_w_what(
                        ESErrorKind::LineError,
                        format!("`=` misplaced"),
                    ));
                }
            }
            None => {
                return Err(ESError::new_w_what(
                    ESErrorKind::LineError,
                    format!("`=` misplaced"),
                ))
            }
            _ => {
                return Err(ESError::new_w_what(
                    ESErrorKind::LineError,
                    format!("Char {} is not allowed after `=`", tmp.unwrap()),
                ))
            }
        }
        Ok(())
    }

    fn lexer(line: &String) -> Result<(ESLineType, Vec<Token>), ESError> {
        let mut lt: Option<ESLineType> = None;
        let mut iter = line.chars().peekable();
        let mut cursor: Option<char>;
        let mut neg: (Vec<bool>, bool) = (Vec::new(), false); // Init the neg
        let mut res: Vec<Token> = Vec::new();

        cursor = iter.next();
        while cursor.is_some() {
            let c = cursor.unwrap();
            match c {
                '!' => neg.1 = !neg.1, // NOT Operator
                'A'..='Z' => {
                    ESLine::lexer_factual(&c, &lt, &mut res, &mut neg)?
                } // Fact
                '(' | ')' => ESLine::lexer_indent(&c, &mut res, &mut neg)?, // Indent
                '+' | '|' | '^' => {
                    ESLine::lexer_computable(&c, &mut res, &mut neg)?
                } // Computable
                '#' => {
                    // Comment
                    if res.len() == 0 && lt.is_none()
                    // If the line is empty of tokens
                    {
                        return Ok((ESLineType::Empty, res));
                    }
                    break;
                }
                '?' => ESLine::lexer_set_line_type(
                    &mut lt,
                    &neg.1,
                    ESLineType::Query,
                )?,
                '=' => ESLine::lexer_handle_equal(
                    &c, &neg.1, &mut iter, &mut lt, &mut res,
                )?, // Is it a Fact or a rule '=>' ?
                '\u{0009}'..='\u{000D}' | '\u{0020}' => (), // Whitespace characters
                _ => {
                    return Err(ESError::new_w_what(
                        ESErrorKind::UnknownOp,
                        format!("Char {} is not allowed", c),
                    ))
                }
            }
            cursor = iter.next();
        }
        if neg.1 {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                String::from("Can't have a dangling `!`"),
            ));
        }
        if lt.is_none() {
            if res.len() == 0 {
                return Ok((ESLineType::Empty, res));
            }
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Line has no type"),
            ));
        }
        Ok((lt.unwrap(), res))
    }

    fn error_logic_token(t1: Option<&Token>, t2: &Token) -> ESError {
        ESError::new_w_what(
            ESErrorKind::LineError,
            format!(
                "Can't use {} before {}",
                match t1 {
                    Some(t) => format!("{}", t),
                    None => format!("<empty>"),
                },
                t2
            ),
        )
    }

    fn check_query_fact(&self) -> ESResult<()> {
        for token in self.tokens.iter() {
            match token {
                Token::Factual(_) => (),
                _ => {
                    return Err(ESError::new_w_what(
                        ESErrorKind::LineError,
                        format!(
                            "Not factual parameter `{}` while {} facts",
                            token,
                            match self.line_type {
                                ESLineType::Fact => "initializing",
                                ESLineType::Query => "querying",
                                _ => "",
                            }
                        ),
                    ))
                }
            }
        }
        Ok(())
    }

    fn check_rule_basic(
        token: &Token,
        has_implied: &mut bool,
        ind_lvl: &mut i32,
    ) -> ESResult<()> {
        match token {
            Token::Behavioral(_op) => *ind_lvl += _op.ind() as i32,
            Token::Computable(_op) => {
                if *has_implied && _op.op_code() != OpCode::And {
                    return Err(ESError::new_w_what(
                        ESErrorKind::LineError,
                        format!("Can't use operator {} after imply", token),
                    ));
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn check_rule_advanced(
        token: &Token,
        prev_token: Option<&Token>,
        has_implied: &mut bool,
        ind_lvl: &mut i32,
        line_type: ESLineType,
    ) -> ESResult<()> {
        match (prev_token, token) // Check token pair
			{
				(None, Token::Behavioral(_op2)) => // None + Behavioral
					match _op2.symbol()
					{
						ModifierType::Imply => // Can't begin with an imply
							return Err(ESLine::error_logic_token(prev_token,
								token)),
						_ => (),
					},
				(None, Token::Computable(_op2)) => // None + Computable
					return Err(ESLine::error_logic_token(prev_token, token)), // Can't begin with an operator
				(Some(Token::Behavioral(_op1)), Token::Behavioral(_op2)) => // Behavioral + Behavioral
					match _op2.symbol()
					{
						ModifierType::Imply => // The second one is an imply
						{
							if *has_implied || line_type != ESLineType::Rule // If it's not a rule or we already implied, fail
							{
								return Err(ESLine::error_logic_token(prev_token,
									token))
							}
							*has_implied = true;
						},
						_ => (),
					},
				(Some(Token::Behavioral(_op1)), Token::Factual(_op2)) => // Behavioral + Factual
					match _op1.symbol()
					{
						ModifierType::Deind => //  `)` before fact fails
							return Err(ESLine::error_logic_token(prev_token,
								token)),
						_ => (), // Do nothing, it's only the imply case
					},
				(Some(Token::Behavioral(_op1)), Token::Computable(_op2)) => // Behavioral + Computable
					match _op1.symbol()
					{
						ModifierType::Ind | ModifierType::Imply => // => `(` before operator fails
							return Err(ESLine::error_logic_token(prev_token,
								token)),
						_ => (), // Do nothing, it's only the imply case
					},
				(Some(Token::Computable(_op1)), Token::Behavioral(_op2)) => // Behavioral + Computable
					match _op2.symbol()
					{
						ModifierType::Deind | ModifierType::Imply => // => `)` after operator fails
							return Err(ESLine::error_logic_token(prev_token,
								token)),
						_ => (), // Do nothing
					},
				(Some(Token::Computable(_op1)), Token::Computable(_op2)) => // Computable + Computable
					return Err(ESLine::error_logic_token(prev_token, token)),
				(Some(Token::Factual(_op1)), Token::Factual(_op2)) => // Factual + Factual
					return Err(ESLine::error_logic_token(prev_token, token)),
				(Some(Token::Factual(_op1)), Token::Behavioral(_op2)) => // Factual + Behavioral
					match _op2.symbol()
					{
						ModifierType::Ind => //  `(` after fact fails
							return Err(ESLine::error_logic_token(prev_token,
								token)),
						_ => (), // Do nothing
					},
				(_, _) => (), // Allow the rest
			}
        Ok(())
    }

    fn check_rule(&self) -> ESResult<()> {
        let mut prev_token: Option<&Token> = None;
        let mut ind_lvl: i32 = 0;
        let mut has_implied: bool = false;

        for token in self.tokens.iter() {
            ESLine::check_rule_basic(token, &mut has_implied, &mut ind_lvl)?;
            ESLine::check_rule_advanced(
                &token,
                prev_token,
                &mut has_implied,
                &mut ind_lvl,
                self.line_type,
            )?;
            prev_token = Some(token);
        }
        if ind_lvl != 0 {
            return Err(ESError::new_w_what(
                ESErrorKind::LineError,
                format!("Uneven number of brackets"),
            ));
        }
        Ok(())
    }

    pub fn check(&self) -> ESResult<()> {
        match self.line_type {
            ESLineType::Empty => (), // Do nothing
            ESLineType::Fact | ESLineType::Query => self.check_query_fact()?, // If a token other than a fact is present, error out
            ESLineType::Rule => self.check_rule()?,
        }
        Ok(())
    }

    fn prefix_brackets(
        token: &Modifier,
        res: &mut Vec<Token>,
        tmp: &mut Vec<Token>,
    ) -> bool {
        match token.symbol() {
            ModifierType::Ind => tmp.push(Token::Behavioral(*token)),
            ModifierType::Deind => loop {
                match tmp.pop() {
                    None | Some(Token::Behavioral(_)) => break,
                    Some(x) => res.push(x),
                }
            },
            _ => return true,
        }
        return false;
    }

    fn prefix_operator(
        token: &Token,
        res: &mut Vec<Token>,
        tmp: &mut Vec<Token>,
    ) {
        loop {
            match tmp.last() {
                None => break,
                Some(_x) => {
                    println!(
                        "Comparing {}({}) > {}({})",
                        token.to_string(),
                        token.priority(),
                        _x.to_string(),
                        _x.priority()
                    );
                    if token.priority() > _x.priority() {
                        break;
                    }
                    res.push(tmp.pop().unwrap());
                }
            }
        }
        tmp.push(*token);
    }

    pub fn to_prefix(self) -> ESResult<Vec<Token>> {
        let mut res: Vec<Token> = Vec::new();
        let mut tmp: Vec<Token> = Vec::new();

        if self.line_type != ESLineType::Rule {
            return Err(ESError::new_w_what(
                ESErrorKind::RPNError,
                String::from("Can't convert to prefix for non-rule line"),
            ));
        }
        for token in self.tokens {
            match token {
                Token::Factual(_x) => res.push(token), // Fact : push to the result stack
                Token::Behavioral(_x) => {
                    if ESLine::prefix_brackets(&_x, &mut res, &mut tmp) {
                        break;
                    }
                }
                Token::Computable(_x) => {
                    ESLine::prefix_operator(&token, &mut res, &mut tmp)
                }
                _ => {
                    return Err(ESError::new_w_what(
                        ESErrorKind::RPNError,
                        format!("Unexpected token {}", token),
                    ))
                }
            }
        }
        while !tmp.is_empty() {
            res.push(tmp.pop().unwrap());
        }
        Ok(res)
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn new(line: &String) -> ESResult<Self> {
        let lexed_line = ESLine::lexer(line)?;
        let res = ESLine {
            line: line.clone(),
            line_type: lexed_line.0,
            tokens: lexed_line.1,
        };
        Ok(res)
    }

    pub fn process(path: &String) -> ESResult<Vec<Self>> {
        let buf: BufReader<File> = BufReader::new(File::open(path)?);
        let mut res: Vec<ESLine> = Vec::new();

        for line in buf.lines() {
            res.push(ESLine::new(&(line?))?);
        }
        Ok(res)
    }
}
