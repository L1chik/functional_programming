use std::path::PathBuf;
use crate::error::ImagicError;
use super::resize::get_image_files;

pub fn get_stats(src: PathBuf) -> Result<(usize, f64), ImagicError> {
    // let image_files = get_image_files(src.to_path_buf())?;
    Ok((4, 5.1))
}