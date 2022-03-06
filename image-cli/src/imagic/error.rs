use std::{io};
use std::path::PathBuf;
use thiserror::Error;

pub type GetFilesResult = Result<Vec<PathBuf>, ImagicError>;

#[derive(Error, Debug)]
pub enum ImagicError {
    #[error("Error in file IO")]
    FileIO(#[from] io::Error),

    // #[error("Invalid user input")]
    // UserInputError(#[from] io::ErrorKind),

    #[error("Error in image processing")]
    ImageResizing(#[from] image::ImageError),

    #[error("Error occurred: {0}")]
    Format(String)
}

// impl fmt::Display for ImagicError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", ImagicError::FormatError("Error occurred".to_string()))
//     }
// }


impl From<io::ErrorKind> for ImagixError {
    fn from(_error: io::ErrorKind) -> Self {
        ImagixError::UserInputError("Error in user input".to_string())
    }
}
