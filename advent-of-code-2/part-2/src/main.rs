use std::path::Path;
use shared_code::{ARRAY, get_file_lines, get_games_iter};

fn main() {
    let mut sum = 0;
    for line in get_file_lines(Path::new("E:/workspaces/advent-of-code-2023/advent-of-code-2/input.txt")) {
            if let Ok(some_line) = line {
                println!("Processing {}", some_line);
                let result = get_minimum_balls(some_line);
                println!("power is {}", result);
                sum += result;
            }
        }

    println!("Finished with total sum: {}", sum);
}

fn get_minimum_balls(input: String) -> i32 {
    let min_blue: &mut Option<i32> = &mut None;
    let min_red: &mut Option<i32> = &mut None;
    let min_green: &mut Option<i32> = &mut None;

    for item in get_games_iter(input.clone().as_str()) {
        get_minimum_count(item, min_green, min_red, min_blue);
    }

    return (*min_blue).unwrap() * (*min_red).unwrap() * (*min_green).unwrap();
}

fn get_minimum_count(line: &str, min_green: &mut Option<i32>, min_red: &mut Option<i32>, min_blue: &mut Option<i32>) {
    for ball in ARRAY.iter() {
        let test = (*ball).colour.captures(line);
        if let Some(another_test) = test {
            let count = &another_test[1];
            let count_integer = count.parse::<i32>().unwrap();
            let id = &another_test[2];
            if id == "green" && (*min_green).is_none() {
                *min_green = Some(count_integer);
            } else if id == "green" && count_integer > (*min_green).unwrap() {
                *min_green = Some(count_integer);
            } else if id =="red" && (*min_red).is_none() {
                *min_red = Some(count_integer);
            } else if id == "red" && count_integer > (*min_red).unwrap() {
                *min_red = Some(count_integer);
            } else if id == "blue" && (*min_blue).is_none() {
                *min_blue = Some(count_integer);
            } else if id == "blue" && count_integer > (*min_blue).unwrap() {
                *min_blue = Some(count_integer);
            }
        }
    }
}