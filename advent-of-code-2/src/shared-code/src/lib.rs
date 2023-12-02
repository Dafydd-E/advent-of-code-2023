use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref RED: Balls = Balls{id: String::from("red"), colour: Regex::new(r"(\d*) (red)").unwrap(), max_count: 12};
    pub static ref GREEN: Balls = Balls{id: String::from("green"), colour: Regex::new(r"(\d*) (green)").unwrap(), max_count: 13};
    pub static ref BLUE: Balls = Balls{id: String::from("blue"), colour: Regex::new(r"(\d*) (blue)").unwrap(), max_count: 14};
}

lazy_static!{
    pub static ref ARRAY: Vec<&'static Balls> = vec![&RED, &GREEN, &BLUE];
}

pub struct Balls {
    id: String,
    pub colour: Regex,
    pub max_count: i32
}

lazy_static! {
    pub static ref TEST_REGEX: Regex = Regex::new(r"Game (\d*): ").unwrap();
}

pub fn get_games_iter(line: &str) -> impl Iterator<Item=&str> {
    let binding = line.split(": ").last();
    if binding.is_some() {
        return binding.unwrap().split("; ")
            .flat_map(|x| x.split(", "));
    }

    panic!("Split failed unexpectedly");
}

pub fn get_file_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Ok(ok_file) = file {
        let buffer = BufReader::new(ok_file);
        return buffer.lines();
    }

    panic!("Error occurred getting file");
}