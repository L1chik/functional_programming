use thiserror::Error;
use std::{fmt, io};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Error, Debug, )]
pub enum ParseError {
    #[error("Unable to parse expression.")]
    UnableToParse(),

    #[error("Invalid character.")]
    InvalidOperator(),

    #[error("Can't read character")]
    WrongParen(String),
}


impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_: Box<dyn Error>) -> Self {
        return ParseError::UnableToParse();
    }
}