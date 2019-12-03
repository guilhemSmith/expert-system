pub mod lexer_parser;
pub mod resolving;
mod state_process;
pub mod utils;

use utils::error::{ESError, ESResult};

fn main() -> ESResult<()> {
    let mut args: Vec<String> = std::env::args().collect();
    let mut err: Option<ESError> = None;

    if args.len() < 2 {
        return Err(ESError::failed_execution(String::from(
            "No file to read :(",
        )));
    }
    args.remove(0);
    if args.len() == 1 {
        let filename = args.pop().unwrap();
        if let Err(new_err) = state_process::run(filename.clone()) {
            eprintln!("{}", new_err);
            err = Some(ESError::failed_execution(format!(
                "{}: execution failed",
                filename
            )));
        };
    } else {
        for filename in args {
            println!("\nfile: {}", filename);
            if let Err(new_err) = state_process::run(filename) {
                eprintln!("{}", new_err);
                if let None = err {
                    err = Some(ESError::failed_execution(String::from(
                        "1 file or more failed.",
                    )));
                }
            };
        }
    }
    match err {
        None => Ok(()),
        Some(err) => Err(err),
    }
}
