use std::fmt::Error;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let _ = get_chars();
}

fn get_chars() -> Result<i32, Error> {
    let mut sum_integer = 0;
    let file = File::open(Path::new("E:/workspaces/advent-of-code-1/src/input.txt"));
    if let Ok(file_ok) = file {
        let reader = BufReader::new(file_ok);
        for line in reader.lines() {
            if let Ok(some_line) = line {
                let (first, last) = get_chars_from_string(some_line.to_lowercase().clone());
                let sum = format!("{}{}", first.value, last.value);

                println!("{}",
                         some_line);

                let integer = sum.parse::<i32>().unwrap();
                sum_integer += integer;
                println!("Sum for this line: {}", integer)
            }

            println!("Total sum: {}", sum_integer);

        }
    }
    return Ok(sum_integer);
}

fn parse_char(character: char) -> i32 {
    if character == char::default() {
        return 0;
    }

    let unsigned = character.to_digit(10).ok_or(0).unwrap();
    return unsigned as i32;
}

static ARRAY: &'static [&Numbers] = &[
    &Numbers{ search: "one", integer_value: 1},
    &Numbers{ search: "two", integer_value: 2},
    &Numbers{ search: "three", integer_value: 3},
    &Numbers{ search: "four", integer_value: 4},
    &Numbers{search: "five", integer_value: 5},
    &Numbers{ search: "six", integer_value: 6},
    &Numbers{search: "seven", integer_value: 7},
    &Numbers{search: "eight", integer_value: 8},
    &Numbers{ search: "nine", integer_value: 9},
    &Numbers{search: "1", integer_value: 1},
    &Numbers{ search: "2", integer_value: 2},
    &Numbers{ search: "3", integer_value: 3},
    &Numbers{ search: "4", integer_value: 4},
    &Numbers{ search: "5", integer_value: 5},
    &Numbers{ search: "6", integer_value: 6},
    &Numbers{ search: "7", integer_value: 7},
    &Numbers{ search: "8", integer_value: 8},
    &Numbers{ search: "9", integer_value: 9}];

fn get_chars_from_string(line: String) -> (Find, Find) {
    let mut first = Find::default();
    let mut last= Find::default();

    for search in ARRAY {
        let matches = line.match_indices((*search).search);
        let last_match = matches.clone().max_by_key(|x| x.0);
        let first_match = matches.clone().min_by_key(|x| x.0);
        if last_match.is_some() {
            if first == Find::default() && last == Find::default() {
                first = Find {index: first_match.unwrap().0, value: (*search).integer_value};
                last = Find {index: last_match.unwrap().0, value: (*search).integer_value};
                continue;
            }

            let index = first_match.unwrap().0;
            if index < first.index {
                first = Find { index: index, value: (*search).integer_value};
            }

            let find_last_index = last_match.unwrap().0;
            if find_last_index > last.index {
                last = Find {index: find_last_index, value:(*search).integer_value};
            }
        }
    }

    return (first, last);
}

struct Numbers<'a> {
    pub search: &'a str,
    pub integer_value: i32
}

#[derive(Default, PartialEq)]
struct Find {
    pub index: usize,
    pub value: i32,
}