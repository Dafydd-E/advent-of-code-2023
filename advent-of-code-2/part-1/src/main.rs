use shared_code::{ARRAY, get_file_lines, get_games_iter, TEST_REGEX};

fn main() {
    get_sum();
}

fn get_sum() {
    let mut sum = 0;
    for line in get_file_lines("E:/workspaces/advent-of-code-2023/advent-of-code-2/input.txt") {
        if let Ok(some_line) = line {
            println!("Processing {}", some_line);
            let result = get_count_from_possible_game(some_line);
            if result.is_some() {
                sum += result.unwrap();
                println!("Added to sum {}", result.unwrap());
                println!("Sum is {}", sum);
            } else {
                println!("Game not possible");
            }
        }
    }

    println!("Finished with total sum: {}", sum);
}
fn get_count_from_possible_game(input: String) -> Option<i32> {
    let game_id = get_game_id(input.clone());
    for item in get_games_iter(input.clone().as_str()) {
        let is_possible = get_possible(item);
        if is_possible.is_none() {
            return None
        }
    }

    if game_id.is_some() {
        println!("Possible game id: {}", game_id.unwrap());
        return Some(game_id.unwrap());
    }

    return None;
}

fn get_game_id(input: String) -> Option<i32> {
    let reg = TEST_REGEX.captures(input.as_str());
    if reg.is_some() {
        let capture = &reg.unwrap()[1];
        return Some(capture.parse::<i32>().unwrap());
    }

    return None;
}

fn get_possible(item: &str) -> Option<bool> {
    for balls in ARRAY.iter() {
        // Could have a game where both colours are selected
        let test = (*balls).colour.captures(item);
        if test.is_some() {
            let count = &test.unwrap()[1];
            let count_as_integer = count.parse::<i32>().unwrap();
            if count_as_integer > balls.max_count {
                return None;
            }
        }
    }

    return Some(true);
}