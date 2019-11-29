pub mod lexer_parser;
pub mod resolving;
pub mod utils;
fn main() -> Result<(), utils::error::ESError> {
    let args: Vec<String> = std::env::args().collect();

    lexer_parser::ESLine::process(&String::from("example.txt"))?;
    Ok(())
}
