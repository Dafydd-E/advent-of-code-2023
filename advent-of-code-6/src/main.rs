use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use regex::Regex;


fn main() {
    let regex: Regex = Regex::new(r"\w?(\d*)*\w?").unwrap();
    let lines = get_file_lines("E:/workspaces/advent-of-code-2023/advent-of-code-6/src/input.txt");
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];
    for (line_index, line) in lines.enumerate() {
        if let Ok(ok_line) = line {
            let mut split = ok_line.split("Time:").last().unwrap();
            let mut number: String = String::default();
            for character in split.chars() {
                if character.is_whitespace() {
                    continue;
                }

                if character.is_digit(10) {
                    number = format!("{}{}", number.to_string(), character);
                }
            }

            if line_index == 0 {
                times.push(number.parse::<usize>().unwrap());
            } else {
                distances.push(number.parse::<usize>().unwrap())
            }
        }
    }

    dbg!(&times);
    dbg!(&distances);
    let mut accum = 1;
    for (index, time) in times.iter().enumerate() {
        let record_distance = distances.get(index).unwrap();
        let mut differentWaysToWin = 0;
        // Hold for 1, travel for (time - 1) at speed of 1/ms
        // For what x, travel for (time - x) at speed of x/ms to be >= y
        // (time - x) * x >= y where x is speed and y is distance and  x >= 0 and x <= time
        let mut game_index: usize = 0;
        loop {
            if game_index > *time {
                break;
            }

            if (*time - game_index) * game_index >= *record_distance {
                differentWaysToWin += 1;
            }


            game_index += 1;
        }

        accum = accum * differentWaysToWin;

    }

    println!("{}", accum);
}

fn day_1() {
    let regex: Regex = Regex::new(r"\w?(\d*)*\w?").unwrap();
    let lines = get_file_lines("E:/workspaces/advent-of-code-2023/advent-of-code-6/src/input.txt");
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];
    for (line_index, line) in lines.enumerate() {
        if let Ok(ok_line) = line {
            let mut split = ok_line.split("Time:").last().unwrap();
            let mut number: String = String::default();
            for character in split.chars() {
                if character.is_whitespace() {
                    if line_index == 0 && number != String::default() {
                        times.push(number.parse::<usize>().unwrap());
                    } else if number != String::default() {
                        distances.push(number.parse::<usize>().unwrap())
                    }

                    number = String::default();
                    continue;
                }

                if character.is_digit(10) {
                    number = format!("{}{}", number.to_string(), character);
                }
            }
             if line_index == 0 {
                 times.push(number.parse::<usize>().unwrap());
             } else {
                 distances.push(number.parse::<usize>().unwrap())
             }
        }
    }

    let mut accum = 1;
    for (index, time) in times.iter().enumerate() {
        let record_distance = distances.get(index).unwrap();
        let mut differentWaysToWin = 0;
        // Hold for 1, travel for (time - 1) at speed of 1/ms
        // For what x, travel for (time - x) at speed of x/ms to be >= y
        // (time - x) * x >= y where x is speed and y is distance and  x >= 0 and x <= time
        let mut game_index: usize = 0;
        loop {
            if game_index > *time {
                break;
            }

            if (*time - game_index) * game_index >= *record_distance {
                differentWaysToWin += 1;
            }


            game_index += 1;
        }

        accum = accum * differentWaysToWin;

    }

    println!("{}", accum);
}

fn get_speed_from_distance_and_time(time: usize, distance: usize) -> f32 {
    return (distance as f32) / (time as f32);
}

pub fn get_file_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Ok(ok_file) = file {
        let reader = BufReader::new(ok_file);
        return reader.lines();
    }

    panic!("File not available");
}
