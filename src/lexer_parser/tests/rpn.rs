use crate::lexer_parser::{ESLine, ESResult};
#[test]
fn infix_to_prefix_easy() -> ESResult<()> {
    let lines: Vec<(String, String)> = vec![
        (format!("A+B+C=>D"), format!("AB+C+")),
        (format!("A|B+C=>D"), format!("ABC+|")),
        (format!("A|B^C=>D"), format!("ABC^|")),
        (format!("A|B^C+D=>E"), format!("ABC^D+|")),
    ];

    for line in lines {
        let tmp = ESLine::new(&line.0)?;

        let mut rpn_str = String::new();
        for t in tmp.to_prefix()? {
            rpn_str.push_str(&t.to_string());
        }
        assert_eq!(rpn_str, line.1);
    }
    Ok(())
}

#[test]
fn infix_to_prefix_hard() -> ESResult<()> {
    let lines: Vec<(String, String)> = vec![
        (format!("A+(((((((B+C)))))))=>D"), format!("ABC++")),
        (format!("(A|B)^C+(D|A)=>E"), format!("AB|C^DA|+")),
    ];

    for line in lines {
        let tmp = ESLine::new(&line.0)?;

        let mut rpn_str = String::new();
        for t in tmp.to_prefix()? {
            rpn_str.push_str(&t.to_string());
        }
        assert_eq!(rpn_str, line.1);
    }
    Ok(())
}

#[test]
fn infix_to_prefix_error() -> ESResult<()> {
    let lines: Vec<String> = vec![format!("=ABC"), format!("?ABC")];

    for line in lines {
        let tmp = ESLine::new(&line)?;
        match tmp.to_prefix() {
            Ok(_x) => panic!("Should have failed at : {} !", line),
            Err(_x) => (),
        }
    }
    Ok(())
}
