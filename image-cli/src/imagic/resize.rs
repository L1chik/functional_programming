use std::path::PathBuf;
use std::{fs, io};
use crate::error::ImagicError;


pub fn resize_request() {
    todo!()
}

fn resize_single() {
    todo!()
}

fn resize_all() {
    todo!()
}

fn resize_image() {
    todo!()
}
fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImagicError> {
    let entries = fs::read_dir(src_folder)
        .map_err(|error| ImagicError::FileIOError(error))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("jpg".as_ref())
                ||  r.extension() == Some("png".as_ref())
        })
        .collect();

    Ok(entries)
}
