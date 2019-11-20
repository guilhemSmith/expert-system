use crate::utils::error::ESError;

pub enum LineType {
    Rule,
    Fact,
    Query,
}

fn get_line(line: &mut String) -> Result<(), ESError> {
    // Get the line without comment
    Ok(())
}

fn get_line_type(line: &String) -> Result<LineType, ESError> {
    //Get line type
    Ok(LineType::Fact) //TODO CHANGE
}

pub fn process_file(path: &String) -> Result<(), ESError> {
    let file = std::fs::File::open(path)?;
    Ok(())
}
