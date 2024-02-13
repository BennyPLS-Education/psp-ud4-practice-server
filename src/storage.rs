use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::{io, process};

/// Opens a file in write-only mode.
///
/// If the file does not exist, it will be created.
/// If the file already exists, it will be erased.
pub fn create(path: &PathBuf) -> File {
    match File::create(&path) {
        Ok(fs) => fs,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!(
                    "{} : Could not create {:?} file, permission denied.",
                    "ERROR",
                    &path.to_str().unwrap_or("{unknown route}")
                );
                process::exit(1);
            }

            _ => {
                eprintln!(
                    "{} : Could not create {:?} file, {}.",
                    "ERROR",
                    &path.to_str().unwrap_or("{unknown route}"),
                    error.kind()
                );
                process::exit(1);
            }
        },
    }
}

/// Saves a string to a file in the given path.
///
/// If the file already exists, it will be overwritten.
/// If the file does not exist, it will be created.
pub(crate) fn save(data: &str, path: &PathBuf) -> io::Result<()> {
    let mut file = create(path);
    file.write_all(data.as_bytes())?;

    Ok(())
}

/// Loads a string from a file in the given path.
///
/// If the file does not exist, it will be created.
pub(crate) fn load(path: &PathBuf) -> String {
    match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_default();
            contents
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                create(&path);
                String::new()
            }

            ErrorKind::PermissionDenied => {
                eprintln!(
                    "{} : Could not open {:?} file, permission denied.",
                    "ERROR",
                    &path.to_str().unwrap_or("{unknown route}")
                );
                process::exit(1);
            }

            _ => {
                eprintln!(
                    "{} : Could not open {:?} file, {}.",
                    "ERROR",
                    &path.to_str().unwrap_or("{unknown route}"),
                    error.kind()
                );
                process::exit(1);
            }
        },
    }
}
