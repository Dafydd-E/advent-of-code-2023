use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub mod part_number;

pub fn get_file_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Ok(ok_file) = file {
        let reader = BufReader::new(ok_file);
        return reader.lines()
    }

    panic!("File not available");
}
