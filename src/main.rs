use std::{env, ffi::OsStr, io::Error, path::{Path, PathBuf}};

fn print_osstring(osstr: &OsStr) {
    if let Some(as_str) = osstr.to_str() {
        println!("{as_str}");
    } else {
        eprintln!("WARNING: invalid utf8 sequence in path, skipping entry...");
    }
}

fn traverse_dir(dir: &Path) -> Result<(), Error> {
    for entry in dir.read_dir()? {
        match entry {
            Ok(entry) => {
                print_osstring(&entry.file_name());
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let dir = env::current_dir()?;
    println!("Hello, '{}'!", dir.to_str().expect("invalid utf8 sequence"));

    traverse_dir(&dir)
    // Ok(())
}
