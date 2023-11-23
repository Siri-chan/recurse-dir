use std::{io, path};

pub type RecurseResult<T> = Result<T, RecurseError>;

#[derive(Debug)]
pub enum RecurseError {
    NotADirectory,
    IOErr(io::Error),
}

pub fn recurse_find(
    root_dir: &path::PathBuf,
    condition: fn(&path::PathBuf) -> bool,
) -> RecurseResult<Vec<path::PathBuf>> {
    let mut matches: Vec<path::PathBuf> = Vec::new();
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
            match recurse_find(&path, condition) {
                Ok(mut vec) => matches.append(&mut vec),
                Err(e) => return Err(e),
            };
        } else if condition(&path) {
            matches.push(path);
        }
    }
    return Ok(matches);
}

#[cfg(test)]
mod tests {
    use crate::recurse_find;

    #[test]
    fn test() {
        assert_eq!(
            vec![std::path::PathBuf::from(".\\Cargo.toml")],
            recurse_find(&std::path::PathBuf::from("."), |path| {
                return path.extension().is_some_and(|s| s.to_str() == Some("toml"));
            })
            .unwrap()
        )
    }
}
