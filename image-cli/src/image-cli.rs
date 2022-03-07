mod imagic;
use crate::imagic::error::ImagicError;
use crate::imagic::resize::{resize_request, Mode, SizeOption};
use crate::imagic::stats::get_stats;

use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;


/// STRUCTURES ///
#[derive(StructOpt, Debug)]
#[structopt(
    name = "resize",
    help = "For help type imagecli resize --help or imagecli stats --help"
)]

enum Commandline {
    #[structopt(help = "\
    Specifu size(small/medium/large), mode(single/all) and srcfolder")]

    Resize {
        #[structopt(long)]
        size: SizeOption,
        #[structopt(long)]
        mode: Mode,
        #[structopt(long, parse(from_os_str))]
        src_folder: PathBuf,
    },

    #[structopt(help = "Specify srcfolder")]
    Stats {
        #[strucopt(long, parse(from_os_str))]
        src_folder: PathBuf,
    },
}

fn main() {
    let args: Commandline = Commandline::from_args();

    match args {
        Commandline::Resize {
            size,
            mode,
            mut src_folder,
        } => {
            match resize_request(size, mode, &mut src_folder) {
                Ok(_) => println!("Image resized successfully"),
                Err(e) => match e {
                    ImagicError::FileIO(e) => println!("{}", e),
                    ImagicError::UserInputError(e) => println!("{}", e),
                    ImagicError::ImageResizing(e) => println!("{}", e),
                    _ => println!("Error in processing"),
                },
            };
        }

        Commandline::Stats { src_folder } => match
            get_stats(src_folder) {
                Ok((count, size)) => println!(
                    "Found {:?} image files with aggregate size of {:?} Mb", count, size
                ),
                Err(e) => match e {
                    ImagicError::FileIO(e) => println!("{}", e),
                    ImagicError::UserInputError(e) => println!("{}", e),
                    _ => println!("Error in processing"),
            },
        },
    }
}


