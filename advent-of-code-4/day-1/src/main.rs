use std::collections::HashSet;
use std::path::Path;
use shared_code::{get_file_lines, parse_into_set, parse_line_into_winning_and_mine};

fn main() {
    let mut sum = 0;
    for (index, line) in get_file_lines(Path::new("E:/workspaces/advent-of-code-2023/advent-of-code-4/input.txt")).enumerate() {
        if let Ok(ok_line) = line {
            let (winning_text, my_text) = parse_line_into_winning_and_mine(ok_line);
            let winning_set = parse_into_set(winning_text);
            let my_set = parse_into_set(my_text);
            let set = winning_set.intersection(&my_set);
            let matches = set.count();
            let points = match matches {
                0 => 0,
                1 => 1,
                _ => 2_i32.pow((matches - 1) as u32)
            };
            sum += points;
        }
    }

    println!("Sum: {}", sum);
}


