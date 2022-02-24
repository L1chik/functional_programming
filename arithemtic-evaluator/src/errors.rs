use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unable to parse expression.")]
    UnableToParse(),

    #[error("Invalid character.")]
    InvalidOperator(),

    #[error("Can't read character")]
    WrongParen(String),
}
