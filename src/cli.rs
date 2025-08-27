use std::fs::{DirEntry, Metadata};
use std::io::Error;
use std::os::unix::fs::MetadataExt;

const KI_B: u64 = 1024;
const MI_B: u64 = 1024 * 1024;
const GI_B: u64 = 1024 * 1024 * 1024;
const TI_B: u64 = 1024 * 1024 * 1024 * 1024;
const PI_B: u64 = 1024 * 1024 * 1024 * 1024 * 1024;

const UNIT_SUFFIXES: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

pub struct ByteSizeUnit {
    pub size: u64,
}

impl ByteSizeUnit {
    pub fn new() -> ByteSizeUnit {
        ByteSizeUnit { size: 0 }
    }

    pub fn create(size: u64) -> ByteSizeUnit {
        ByteSizeUnit { size }
    }

    pub fn string_tuple(&self) -> (String, String) {
        let mut suffix_idx = 0;
        let mut real_num = self.size as f64;

        while real_num > 1024.0 && suffix_idx < 5 {
            real_num /= 1024.0;
            suffix_idx += 1;
        }

        if real_num.fract() >= 0.01 {
            (format!("{real_num:.2}"), UNIT_SUFFIXES[suffix_idx].to_string())
        } else {
            ((real_num as u32).to_string(), UNIT_SUFFIXES[suffix_idx].to_string())
        }
    }
}

pub struct EntrySize {
    pub logical: ByteSizeUnit,
    pub physical: ByteSizeUnit,
}

impl EntrySize {
    pub fn create(metadata: &Metadata) -> EntrySize {
        EntrySize {
            logical: ByteSizeUnit {
                size: metadata.len(),
            },
            physical: ByteSizeUnit {
                size: metadata.blocks() * 512,
            },
        }
    }
}

fn size_as_str(size: Option<u64>) -> String {
    match size {
        Some(size) => size.to_string(),
        None => String::from("?"),
    }
}

pub fn print_entry(entry: &DirEntry, size: Option<EntrySize>) -> Result<(), Error> {
    let raw_filename = entry.file_name();
    let raw_filename = raw_filename.to_str().unwrap_or("???");

    let mut filename = String::from(raw_filename);
    filename.truncate(32);
    if filename.len() < raw_filename.len() {
        if let Some((idx, _)) = filename.char_indices().nth_back(2) {
            filename.replace_range(idx.., "...");
        }
    }

    let (lsize, psize) = match size {
        Some(size) => (size.logical.string_tuple(), size.physical.string_tuple()),
        None => (
            (String::from("?"), String::from("B")),
            (String::from("?"), String::from("B")),
        ),
    };

    println!(
        " - {filename:32}\tlog: {:>10} {:<5}\tphy: {:>10} {:<5}",
        lsize.0, lsize.1, psize.0, psize.1
    );

    Ok(())
}
