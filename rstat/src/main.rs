mod error;
mod srcstats;

use std::path::PathBuf;
use structopt::StructOpt;
use srcstats::get_summary_src_stats;
use error::StatsError;

/// TODO: USE GPIO IN TG2560 PROJECT

#[derive(Debug, StructOpt)]
#[structopt(
    name = "RStat",
    about = "This tool generates statistics of Rust projects",
)]

struct Opt {
    #[structopt(name = "source directory", parse(from_os_str))]
    dir: PathBuf,

    #[structopt(name = "mode", long, short = "m")]
    mode: String,
}

fn main() -> Result<(), StatsError> {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];

    match mode {
        "src" => {
            let stats = get_summary_src_stats(&opt.dir)?;
            println!("Total stats: {:?}", stats)
        }
        _ => println!("No stats!"),
    }

    Ok(())
}
