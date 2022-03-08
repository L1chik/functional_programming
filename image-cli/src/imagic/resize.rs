use std::path::{PathBuf};
use std::{fs, io, fmt};
use std::fmt::Formatter;
use std::str::FromStr;
use std::time::{Instant, Duration};
use super::error::{ImagicError, GetFilesResult};
use image::{ImageFormat};


/// STRUCTURES ///
pub struct Elapsed(Duration);

#[derive(Debug, PartialEq)]
pub enum Mode {
    Single,
    All,
}

#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}


/// IMPLEMENTATIONS ///
impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1_000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1_000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

impl FromStr for SizeOption {
    type Err = ImagicError;

    fn from_str(size: &str) -> Result<Self, Self::Err> {
        match size {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Ok(SizeOption::Small) // Default value
        }
    }
}

impl FromStr for Mode {
    type Err = ImagicError;

    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        match mode {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagicError::UserInputError("Wrong value for mode".to_string())),
        }
    }
}

pub fn resize_request(size: SizeOption, mode: Mode, src: &mut PathBuf) -> Result<(), ImagicError> {
    let size: u32 = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };
    let _ = match mode {
        Mode::All => resize_all(size, src)?,
        Mode::Single => resize_single(size, src)?,
    };

    Ok(())
}

fn resize_single(size: u32, src: &mut PathBuf) -> Result<(), ImagicError> {
    resize_image(src, size)?;

    Ok(())
}

fn resize_all(size: u32, src: &mut PathBuf) -> Result<(), ImagicError> {
    if let Ok(entries) = get_image_files(src.to_path_buf()) {
        for mut entry in entries {
            resize_image(&mut entry, size)?;
        }
    };

    Ok(())
}

pub fn resize_image(src: &mut PathBuf, size: u32) -> Result<(), ImagicError> {
    let mut img_name = src.clone();
    img_name.set_extension("png");

    let new_name = img_name
        .file_name()
        .unwrap()
        .to_str()
        .ok_or(io::ErrorKind::InvalidInput); // Transform Option into a Result, Some->Ok | None->Err(err)

    let mut dest_folder = src.clone();
    dest_folder.pop();
    dest_folder.push("tmp/");

    if !dest_folder.exists() {
        fs::create_dir(&dest_folder)?;
    }

    dest_folder.pop();
    dest_folder.push("tmp/tmp.png");
    dest_folder.set_file_name(new_name?);

    let timer = Instant::now();
    let img = image::open(&src)
        .expect("Read image failed");

    let img_converted = img.thumbnail_exact(size, size);
    let mut output = fs::File::create(&dest_folder)?;
    img_converted.write_to(&mut output, ImageFormat::Png)?;

    println!("Thumbnailed file: {:?} to size {}x{} in {}. Output file
    in {:?}", src, size, size, Elapsed::from(&timer), dest_folder);

    Ok(())
}

pub fn get_image_files(src_folder: PathBuf) -> GetFilesResult {
    let entries = fs::read_dir(src_folder)
        .map_err(ImagicError::FileIO)?
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


/// TESTS ///
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reading_file() {
        let path = PathBuf::from("img/");
        let test = get_image_files(path).unwrap();

        assert_eq!(true, test.contains(&PathBuf::from("img/gleb1.jpg")));
        assert_eq!(true, test.contains(&PathBuf::from("img/gleb2.jpg")));
        assert_eq!(false, test.contains(&PathBuf::from("img/gashishishpak.gif")));
    }

    #[test]
    fn test_single_image_resize() {
        let mut path = PathBuf::from("img/gleb1_2.jpg");
        let dest_path = PathBuf::from("img/tmp/gleb1_2.png");

        match resize_request(SizeOption::Small, Mode::Single, &mut path) {
            Ok(_) => println!("Resize of single image completed"),
            Err(e) => println!("Error occurred: {:?}", e),
        }

        assert_eq!(true, dest_path.exists());
    }

    #[test]
    fn test_all_image_resize() {
        let mut path = PathBuf::from("img/");
        let dest_path1 = PathBuf::from("img/tmp/gleb1.png");
        let dest_path2 = PathBuf::from("img/tmp/gleb1_2.png");
        let dest_path3= PathBuf::from("img/tmp/gleb2.png");
        let dest_path4= PathBuf::from("img/tmp/gleb23.png");

        let _res = resize_request(SizeOption::Medium, Mode::All, &mut path);

        assert_eq!(true, dest_path1.exists());
        assert_eq!(true, dest_path2.exists());
        assert_eq!(true, dest_path3.exists());
        assert_eq!(true, dest_path4.exists());
    }
}