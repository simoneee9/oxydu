use std::{env, fs::DirEntry, io::Error, os::unix::fs::MetadataExt, path::Path};

mod cli;

fn traverse_dir(dir: &Path) -> Result<(), Error> {
    for entry in (dir.read_dir()?).flatten() {
        let size = match entry.metadata() {
            Ok(metadata) => Some(cli::EntrySize::create(&metadata)),
            Err(_) => None,
        };

        if let Err(e) = cli::print_entry(&entry, size) {
            eprintln!("WARNING: {e}. skipping entry...");
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
