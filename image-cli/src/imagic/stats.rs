use std::path::PathBuf;
use super::error::ImagicError;
use super::resize::get_image_files;

pub fn get_stats(src: PathBuf) -> Result<(usize, f64), ImagicError> {
    let image_files = get_image_files(src.to_path_buf())?;
    let size = image_files
        .iter()
        .map(move |f| f.metadata().unwrap().len())
        .sum::<u64>();

    // (Number of images in folder, total size of all images)
    Ok((image_files.len(), (size / 1000000) as f64))
}