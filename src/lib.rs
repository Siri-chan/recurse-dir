use std::{fs, io, path};

pub type RecurseResult<T> = Result<T, RecurseError>;

pub enum RecurseError {
    NotADirectory,
    IOErr(io::Error),
}

pub fn recurse_find(
    root_dir: path::PathBuf,
    condition: fn(&path::PathBuf) -> bool,
) -> RecurseResult<Vec<fs::File>> {
    let mut matches: Vec<fs::File> = Vec::new();
    if !root_dir.is_dir() {
        return Err(RecurseError::NotADirectory);
    }
    let readdir = match root_dir.read_dir() {
        Ok(readdir) => readdir,
        Err(err) => return Err(RecurseError::IOErr(err)),
    };
    for entry in readdir {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(err) => return Err(RecurseError::IOErr(err)),
        };
        if path.is_dir() {
            match recurse_find(root_dir.clone(), condition) {
                Ok(mut vec) => matches.append(&mut vec),
                Err(e) => return Err(e),
            }
        } else if condition(&path) {
            matches.push(match fs::File::open(path) {
                Ok(f) => f,
                Err(err) => return Err(RecurseError::IOErr(err)),
            })
        }
    }
    return Ok(matches);
}
