// use std::fmt;
use std::io;
use thiserror::Error;

// #[derive(Debug)]
// pub struct StatsError {
//     pub msg: String,
// }
//
// impl fmt::Display for StatsError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Display: {}", self)
//     }
// }
//
// impl From<&str> for StatsError {
//     fn from(line: &str) -> Self {
//         StatsError { msg: line.to_string(), }
//     }
// }
//
// impl From<io::Error> for StatsError {
//     fn from(err: io::Error) -> Self {
//         StatsError {
//             msg: err.to_string(),
//         }
//     }
// }
//
// impl From<std::num::TryFromIntError> for StatsError {
//     fn from(_: std::num::TryFromIntError) -> Self {
//         StatsError {
//             msg: "Num conversion error".to_string(),
//         }
//     }
// }

#[derive(Debug, Error)]
pub enum StatsError {
    #[error("Num conversion error")]
    NumConversion(#[from] std::num::TryFromIntError),

    #[error("Wrong directory")]
    DirectoryIO(#[from] io::Error)
}