use crate::lexer_parser::{ESLine, ESLineType};
use crate::utils::error::ESError;

#[test]
fn lexer_single_line_empty() -> Result<(), ESError> {
    let line = format!("");
    let tmp: ESLine = ESLine::new(&line)?;
    tmp.check()?;
    Ok(())
}

#[test]
fn lexer_single_line_queries_correct() -> Result<(), ESError> {
    let lines: Vec<String> = vec![
        format!("?A"),
        format!("?ABC"),
        format!("?ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        format!("?    C D E F G C S A		T"),
    ];
    for line in lines {
        let tmp: ESLine = ESLine::new(&line)?;
        assert_eq!(tmp.line_type, ESLineType::Query);
        match tmp.check() {
            Ok(_f) => (),
            Err(_e) => {
                panic!("Should NOT have failed here for line `{}`", line)
            }
        }
    }
    Ok(())
}

#[test]
fn lexer_single_line_queries_incorrect() -> Result<(), ESError> {
    let lines: Vec<String> = vec![
        format!("?A+"),
        format!("?A|BC"),
        format!("?^ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
    ];
    for line in lines {
        let tmp: ESLine = ESLine::new(&line)?;
        assert_eq!(tmp.line_type, ESLineType::Query);
        match tmp.check() {
            Ok(_f) => panic!("Should've failed here for line `{}`", line),
            Err(_e) => (),
        }
    }
    Ok(())
}

#[test]
fn lexer_single_line_facts_correct() -> Result<(), ESError> {
    let lines: Vec<String> = vec![
        format!("=A"),
        format!("=ABC"),
        format!("=ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        format!("=C D E F G C S A		T"),
    ];
    for line in lines {
        let tmp: ESLine = ESLine::new(&line)?;
        assert_eq!(tmp.line_type, ESLineType::Fact);
        match tmp.check() {
            Ok(_f) => (),
            Err(_e) => {
                panic!("Should NOT have failed here for line `{}`", line)
            }
        }
    }
    Ok(())
}

#[test]
fn lexer_single_line_facts_incorrect() -> Result<(), ESError> {
    let lines: Vec<String> = vec![
        format!("=A+"),
        format!("=A|BC"),
        format!("=A^BCDEFGHIJKLMNOPQRSTUVWXYZ"),
    ];
    for line in lines {
        let tmp: ESLine = ESLine::new(&line)?;
        assert_eq!(tmp.line_type, ESLineType::Fact);
        match tmp.check() {
            Ok(_f) => panic!("Should've failed here for line `{}`", line),
            Err(_e) => (),
        }
    }
    Ok(())
}

#[test]
fn lexer_single_line_rules_correct() -> Result<(), ESError> {
    let lines: Vec<String> = vec![
        format!("A + B => C"),
        format!("A + (((B + D))) + (A | C ^ B) => C + D + E"),
    ];
    for line in lines {
        let tmp: ESLine = ESLine::new(&line)?;
        assert_eq!(tmp.line_type, ESLineType::Rule);
        match tmp.check() {
            Ok(_f) => (),
            Err(_e) => {
                panic!("Should NOT have failed here for line `{}`", line)
            }
        }
    }
    Ok(())
}

#[test]
fn lexer_single_line_rules_incorrect() -> Result<(), ESError> {
    let lines: Vec<String> = vec![
        format!("+ A B C => E"),
        format!("A + + B C => E"),
        format!("A(B) C => E"),
        format!("A + (B +) + C => E"),
        format!("A + (B + D) + C + => E"),
        format!("A + (B + D) + C ()=> E"),
        format!("A + (B => E"),
        format!("A + (B)) => E"),
        format!("A + (B) )=> E"),
    ];
    for line in lines {
        let tmp: ESLine = ESLine::new(&line)?;
        assert_eq!(tmp.line_type, ESLineType::Rule);
        match tmp.check() {
            Ok(_f) => panic!("Should've failed here for line `{}`", line),
            Err(_e) => (),
        }
    }
    Ok(())
}
