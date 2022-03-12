use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path;
use std::path::{Path, PathBuf};
use crate::error::StatsError;


pub type StatsResult = Result<SrcStats, StatsError>;

#[derive(Debug)]
pub struct SrcStats {
    pub number_of_files: u32,
    pub  loc: u32,
    pub comments: u32,
    pub blanks: u32,
}

fn get_src_stats_for_file(file: &Path) -> StatsResult {
    let file_contents = fs::read_to_string(file)?;
    let mut loc: u32 = 0;
    let mut blanks: u32 = 0;
    let mut comments: u32 = 0;

    for line in file_contents.lines() {
        if line.len() == 0 { blanks += 1; }
        else if line.trim_start().starts_with("//") {
            comments += 1;
        } else {
            loc += 1;
        }
    }

    let source_stats = SrcStats {
        number_of_files: u32::try_from(file_contents.lines().count())?,
        loc,
        blanks,
        comments,
    };

    Ok(source_stats)
}

pub fn get_summary_src_stats(dir: &Path) -> StatsResult{
    let mut total_loc: u32 = 0;
    let mut total_comments: u32 = 0;
    let mut total_blanks: u32 = 0;
    let mut dir_entries: Vec<PathBuf> = vec![dir.to_path_buf()];
    let mut files_entries: Vec<DirEntry> = vec![];

    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().is_dir() {
                    dir_entries.push(entry.path());
                } else {
                    if entry.path().extension() == Some(OsStr::new("rs")) {
                        files_entries.push(entry)
                    }
                }
            }
        }
    }

    let file_count = files_entries.len();

    for entry in files_entries {
        let stat = get_src_stats_for_file(&entry.path())?;

        total_loc += stat.loc;
        total_blanks += stat.blanks;
        total_comments += stat.comments;
    }

    Ok(SrcStats {
        number_of_files: u32::try_from(file_count)?,
        loc: total_loc,
        blanks: total_blanks,
        comments: total_comments,
    })
}