use crate::lexer_parser::{ESLine, ESLineType};
use crate::utils::error::{ESError, ESErrorKind};
use crate::utils::token::*;

#[test]
fn lexer_single_line_empty() -> Result<(), ESError> {
    let line = format!("");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line, line);
    assert_eq!(tmp.line_type, ESLineType::Empty);
    assert_eq!(tmp.tokens.len(), 0);
    Ok(())
}

#[test]
fn lexer_single_line_commented() -> Result<(), ESError> {
    let line = format!("# Hello World je suis une ligne");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line, line);
    assert_eq!(tmp.line_type, ESLineType::Empty);
    assert_eq!(tmp.tokens.len(), 0);
    Ok(())
}

#[test]
fn lexer_single_line_rule() -> Result<(), ESError> {
    let line = format!("A + B => C");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line, line);
    assert_eq!(tmp.line_type, ESLineType::Rule);
    assert_eq!(tmp.tokens.len(), 5);
    if let Token::Factual(_) = tmp.tokens[0] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[0])
    }
    if let Token::Computable(_) = tmp.tokens[1] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[1])
    }
    if let Token::Factual(_) = tmp.tokens[2] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[2])
    }
    if let Token::Behavioral(_) = tmp.tokens[3] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[3])
    }
    if let Token::Factual(_) = tmp.tokens[4] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[4])
    }
    Ok(())
}

#[test]
fn lexer_single_line_query() -> Result<(), ESError> {
    let line = format!("?ABC");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line, line);
    assert_eq!(tmp.line_type, ESLineType::Query);
    assert_eq!(tmp.tokens.len(), 3);
    if let Token::Factual(_op) = tmp.tokens[0] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[0])
    }
    if let Token::Factual(_op) = tmp.tokens[1] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[1])
    }
    if let Token::Factual(_op) = tmp.tokens[2] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[2])
    }
    Ok(())
}

#[test]
fn lexer_single_line_fact() -> Result<(), ESError> {
    let line = format!("=ABC");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line, line);
    assert_eq!(tmp.line_type, ESLineType::Fact);
    assert_eq!(tmp.tokens.len(), 3);
    if let Token::Factual(_op) = tmp.tokens[0] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[0])
    }
    if let Token::Factual(_op) = tmp.tokens[1] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[1])
    }
    if let Token::Factual(_op) = tmp.tokens[2] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[2])
    }
    Ok(())
}

#[test]
fn lexer_single_line_correct_w_comment() -> Result<(), ESError> {
    let line = format!("   =ABC    #   Salut ");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line, line);
    assert_eq!(tmp.line_type, ESLineType::Fact);
    assert_eq!(tmp.tokens.len(), 3);
    if let Token::Factual(_op) = tmp.tokens[0] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[0])
    }
    if let Token::Factual(_op) = tmp.tokens[1] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[1])
    }
    if let Token::Factual(_op) = tmp.tokens[2] {
    } else {
        panic!("Wrong token type ! `{}`", tmp.tokens[2])
    }
    Ok(())
}

#[test]
fn lexer_single_line_error() {
    let lines: Vec<String> = vec![
        format!("= ABC"),
        format!("A+B+C=D"),
        format!("?A+B+C=>D"),
        format!("?=BC"),
        format!("ABC"),
        format!("A+B+C=#"),
    ];

    for line in lines {
        let tmp = ESLine::new(&line);
        match tmp {
            Ok(_) => panic!("Should have failed {} !", line),
            Err(err) => assert_eq!(err.kind(), ESErrorKind::LineError),
        }
    }
}

#[test]
fn lexer_single_unknown_op() {
    let lines: Vec<String> = vec![
        format!("A+B-C=>D"),
        format!("A+B+C=>/"),
        format!("totalement pas correct mon gros"),
    ];

    for line in lines {
        let tmp = ESLine::new(&line);
        match tmp {
            Ok(_) => panic!("Should have failed {} !", line),
            Err(err) => assert_eq!(err.kind(), ESErrorKind::UnknownOp),
        }
    }
}
