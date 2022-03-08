use std::{io};
use std::path::PathBuf;
use thiserror::Error;

pub type GetFilesResult = Result<Vec<PathBuf>, ImagicError>;

#[derive(Error, Debug)]
pub enum ImagicError {
    #[error("Error in file IO")]
    FileIO(#[from] io::Error),

    #[error("Invalid user input")]
    UserInputError(String),

    #[error("Error in image processing")]
    ImageResizing(#[from] image::ImageError),
}

// impl fmt::Display for ImagicError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", ImagicError::FormatError("Error occurred".to_string()))
//     }
// }


impl From<io::ErrorKind> for ImagicError {
    fn from(_error: io::ErrorKind) -> Self {
        ImagicError::UserInputError("Error in user input".to_string())
    }
}
