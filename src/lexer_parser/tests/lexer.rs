use crate::lexer_parser::{ESLine, ESLineType};
use crate::utils::error::{ESError, ESErrorKind, ESResult};
use crate::utils::token::*;

#[test]
fn lexer_single_line_empty() -> Result<(), ESError> {
    let line = format!("");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line_type, ESLineType::Empty);
    assert_eq!(tmp.tokens.len(), 0);
    Ok(())
}

#[test]
fn lexer_single_line_commented() -> Result<(), ESError> {
    let line = format!("# Hello World je suis une ligne");
    let tmp: ESLine = ESLine::new(&line)?;
    assert_eq!(tmp.line_type, ESLineType::Empty);
    assert_eq!(tmp.tokens.len(), 0);
    Ok(())
}

#[test]
fn lexer_single_line_rule() -> Result<(), ESError> {
    let line = format!("A + B => C");
    let tmp: ESLine = ESLine::new(&line)?;
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
fn lexer_single_line_equal_correct() {
    let lines: Vec<String> = vec![
        format!("A+B+C=>     D"),
        format!("=         D"),
        format!("=			D"),
        format!("A + B =	 > C"),
    ];

    for line in lines {
        let tmp = ESLine::new(&line);
        match tmp {
            Ok(_) => (),
            Err(_err) => panic!("Should NOT have failed {} !", line),
        }
    }
}

#[test]
fn lexer_single_line_equal_incorrect() {
    let lines: Vec<String> =
        vec![format!("T=>!D"), format!("A+B="), format!("=!A")];

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

#[test]
fn lexer_neg_correct() {
    let lines: Vec<String> = vec![
        format!("!A+!B+!C=>D"),
        format!("!    	A+B+! C=>D"),
        format!("!!!!!!!!A+B+!C=>D"),
    ];

    for line in lines {
        let tmp = ESLine::new(&line);
        match tmp {
            Ok(_) => (),
            Err(_err) => panic!("Should NOT have failed : {} !", line),
        }
    }
}

#[test]
fn lexer_neg_correct_propagation() -> ESResult<()> {
    let lines: Vec<(String, Vec<bool>)> = vec![
        (format!("!(A+B)=>D"), vec![true, true, false]),
        (format!("!(A+B)+B=>D"), vec![true, true, false]),
        (format!("!(A+B)+!B=>D"), vec![true, true, true]),
        (
            format!("!(A+B+!(C|D))+!B=>D"),
            vec![true, true, false, false, true],
        ),
    ];

    for line in lines {
        let tmp = ESLine::new(&line.0);
        let mut i = 0;
        match tmp {
            Ok(_) => (),
            Err(_err) => {
                panic!("Should NOT have failed : {} !\n{}", line.0, _err)
            }
        }
        for t in tmp.unwrap().tokens().iter() {
            if i >= line.1.len() {
                break;
            }
            match t {
                Token::Factual(_x) => {
                    assert_eq!(
                        _x.negated(),
                        *line.1.get(i).unwrap(),
                        "Problem at index {} with token {}",
                        i,
                        t.to_string()
                    );
                    i += 1;
                }
                _ => continue,
            }
        }
    }
    Ok(())
}

#[test]
fn lexer_neg_incorrect() {
    let lines: Vec<String> = vec![
        format!("A!+B+C=>D"),
        format!("!A+B+!C !=> D"),
        format!("A+B+C=>!D"),
        format!("A+B+C=>D+!F"),
        format!("=!ABC"),
        format!("=A!BC"),
        format!("?AB!C"),
    ];

    for line in lines {
        let tmp = ESLine::new(&line);
        match tmp {
            Ok(_) => panic!("Should have failed {} !", line),
            Err(err) => assert_eq!(err.kind(), ESErrorKind::LineError),
        }
    }
}
