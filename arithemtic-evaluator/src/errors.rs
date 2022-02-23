use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unable to parse expression")]
    UnableToParse(#[from] io::Error),

    #[error("Can't read character")]
    InvalidOperator(),
}
