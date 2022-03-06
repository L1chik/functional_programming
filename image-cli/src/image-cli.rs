mod imagic;
use std::path::PathBuf;
use  crate::imagic::resize;
use std::error::Error;
use std::io;
// use  crate::imagic::error;


fn main() -> Result<(), Box<dyn Error>>{
    let mut path = PathBuf::from("../img/gleb1_2.jpg");
    // let test = resize::get_image_files(path)?;
        // let _res = vec!["../img/gleb1.jpg", "../img/gleb2.jpg"];
    // resize::resize_image(&mut path, 300);
    // let new_name = path
    //     .file_stem() // Return entire file name if it exists
    //     .unwrap()
    //     .to_str()
    //     .ok_or(io::ErrorKind::InvalidInput) // Transform Option into a Result, Some->Ok | None->Err(err)
    //     .map(|f| format!("{}.png", f));
    // path.set_extension("png");
    // let new_name = path.file_name()
    //     .ok_or(io::ErrorKind::InvalidInput);
    let mut dest_folder = path.clone();
    dest_folder.pop();
    dest_folder.push("tmp/");



    println!("{:?}", dest_folder);
    println!("{:?}", path);

    Ok(())
}

