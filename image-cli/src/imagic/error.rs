use std::{fmt, io};
use thiserror::Error;

#[derive(Error)]
pub enum ImagicError {
    #[error("Error in file IO")]
    FileIOError(#[from] io::Error),

    #[error("Invalid user input")]
    UserInputError(#[from] io::ErrorKind),

    #[error("Error in image processing")]
    ImageResizingError(#[from] image::ImageError),

    #[error("Error occurred")]
    FormatError(String)
}

impl fmt::Display for ImagicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ImagicError::FormatError("Error occurred".to_string()))
    }
}