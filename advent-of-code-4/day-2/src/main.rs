use std::path::Path;
use shared_code::{get_file_lines, parse_into_set, parse_line_into_winning_and_mine};

const MAX_ROWS_COUNT: usize = 196;

fn main() {
    let mut vector : Vec<(usize, usize)> = vec![];
    for (index, line) in get_file_lines(Path::new("E:/workspaces/advent-of-code-2023/advent-of-code-4/input.txt")).enumerate() {
        if let Ok(ok_line) = line {
            let matches = play_game(ok_line);

            let exists = vector.get_mut(index);
            let mut play_iterations = 1;
            if let Some(exists_some) = exists {
                play_iterations += exists_some.1;
                exists_some.1 += 1;
            } else {
                vector.push((index, 1));
            }
            for i in 0..play_iterations {
                for some_index in (0..matches) {
                    let this_index = some_index + 1 + index;
                    if this_index >= MAX_ROWS_COUNT {
                        break;
                    }
                    let existing = vector.get_mut(this_index);
                    if let Some(existing_yes) = existing {
                        existing_yes.1 += 1
                    } else {
                        // win a copy and have an original (if we're not the last to process
                        vector.push((this_index, 1));
                    }
                }
            }

        }
    }

    let sum = vector.iter().map(|x| x.1).sum::<usize>();
    println!("Sum: {}", sum);
}

fn play_game(ok_line: String) -> usize {
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
    return matches;
}
