pub mod lexer_parser;
pub mod resolving;
pub mod utils;

fn toto() -> Result<(), utils::error::ESError> {
    let lines: Vec<String> =
        vec![format!("A + (((B + D))) + (A | C ^ B) => C + D + E")];
    for line in lines {
        let tmp: lexer_parser::ESLine = lexer_parser::ESLine::new(&line)?;
        tmp.check()?;
    }
    Ok(())
}
fn main() -> Result<(), utils::error::ESError> {
    let args: Vec<String> = std::env::args().collect();

    toto();
    lexer_parser::ESLine::process(&String::from("example.txt"))?;
    Ok(())
}
