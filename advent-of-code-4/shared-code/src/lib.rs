use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn parse_line_into_winning_and_mine<'a>(line: String) -> (String, String) {
    let game = line.split(": ").last().unwrap().to_string();
    let game_split: Vec<&str> = game.split(" | ").into_iter().collect();
    return (game_split.first().unwrap().to_string(), game_split.last().unwrap().to_string());
}

pub fn get_file_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Ok(ok_file) = file {
        let reader = BufReader::new(ok_file);
        return reader.lines()
    }

    panic!("File not available");
}

pub fn parse_into_set(numbers: String) -> HashSet<usize> {
    let mut number = String::default();
    let mut parsed_numbers: Vec<usize> = vec![];

    for (index, character) in numbers.chars().into_iter().enumerate() {
        if index > 0 && index % 3 == 2 {
            parsed_numbers.push(number.parse::<usize>().unwrap());
            number = String::default();
            continue;
        }

        let out = match character {
            ' ' => '0',
            _ => character
        };

        number = format!("{}{}", number, out)
    }

    if number.len() == 2 {
        parsed_numbers.push(number.parse::<usize>().unwrap())
    }
    return HashSet::from_iter(parsed_numbers.into_iter());
}
