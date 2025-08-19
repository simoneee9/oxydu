use std::{env, ffi::OsStr, fs::DirEntry, io::Error, os::unix::fs::MetadataExt, path::Path};

fn size_as_str(size: Option<u64>) -> String {
    match size {
        Some(size) => size.to_string(),
        None => String::from("?"),
    }
}

fn print_entry(entry: &DirEntry, lsize: Option<u64>, psize: Option<u64>) -> Result<(), Error> {
    let raw_filename = entry.file_name();
    let raw_filename = raw_filename.to_str().unwrap_or("???");


    let mut filename = String::from(raw_filename);
    filename.truncate(32);
    if filename.len() < raw_filename.len() {
        if let Some((idx, _)) = filename.char_indices().nth_back(2) {
            filename.replace_range(idx.., "...");
        }
    }

    let lsize = size_as_str(lsize);
    let psize = size_as_str(psize);

    println!(" - {filename:32}\tlog {lsize:>5} B\tphy {psize:>5} B");

    Ok(())
}

fn traverse_dir(dir: &Path) -> Result<(), Error> {
    for entry in (dir.read_dir()?).flatten() {
        let mut lsize: Option<u64> = None;
        let mut psize: Option<u64> = None;
        if let Ok(metadata) = entry.metadata() {
            if !metadata.is_dir() {
                lsize = Some(metadata.len());
                psize = Some(metadata.blocks() * 512);
            }
        }

        if let Err(e) = print_entry(&entry, lsize, psize) {
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
