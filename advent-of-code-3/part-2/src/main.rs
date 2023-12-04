use std::path::Path;
use shared_code::get_file_lines;
use shared_code::part_number::{PartNumber, read_parts_in_line};

const MIN_COL: usize = 0;
const MAX_COL: usize = 139;

fn main() {
    let mut symbols: Vec<Vec<PartNumber>> = vec![vec![]];
    let mut last_line_part_number: Vec<PartNumber> = vec![];
    let mut line_index = 0;
    for line in get_file_lines(Path::new("E:/workspaces/advent-of-code-2023/advent-of-code-3/input.txt")) {
        let mut next_in_line_symbols: Vec<PartNumber> = vec![];
        let mut next_in_line_part_numbers : Vec<PartNumber> = vec![];
        if let Ok(ok_line) = line {
            println!("Processing line: \n{}", ok_line);
            println!("line index {}", line_index);

            for part in &mut read_parts_in_line(ok_line, line_index) {
                if part.is_symbol && part.is_gear() {
                    let mut next_symbol = PartNumber { line_index,
                        index_span: part.index_span.clone(),
                        is_valid: false,
                        is_symbol: true,
                        string_value: part.string_value.clone(),
                        number_of_attached_gears: part.number_of_attached_gears.clone(),
                        gear_ratio: part.gear_ratio.clone() };

                    if line_index > 0 {
                        for last_line_part in &mut last_line_part_number {
                            if last_line_part.index_span.contains(&(part.index_span.first().unwrap() - 1)) ||
                                last_line_part.index_span.contains(&(part.index_span.first().unwrap())) ||
                                last_line_part.index_span.contains(&(part.index_span.first().unwrap() + 1)) {
                                // found a part in previous_line which touches gear
                                last_line_part.to_owned().mark_as_valid();

                                next_symbol.gear_ratio = next_symbol.gear_ratio * last_line_part.get_integer_value();
                                next_symbol.number_of_attached_gears += 1;
                            }
                        }
                    }

                    next_in_line_symbols.push(next_symbol);
                } else if part.is_symbol == false {
                    if line_index != 0 {
                        let last_line_valid = symbols
                            .last_mut()
                            .unwrap()
                            .iter_mut()
                            .find(|x|
                                part.index_span.contains(&(MIN_COL.max(x.index_span.first().unwrap() - 1))) ||
                                part.index_span.contains(&(x.index_span.first().unwrap())) ||
                                part.index_span.contains(&(MAX_COL.min(x.index_span.first().unwrap() + 1))));

                        if last_line_valid.is_some() {
                            let last_line = last_line_valid.unwrap();
                            last_line.number_of_attached_gears += 1;
                            last_line.gear_ratio = last_line.gear_ratio * part.get_integer_value();
                        }
                    }

                    next_in_line_part_numbers.push(part);
                }
            }
            symbols.push(next_in_line_symbols);
            last_line_part_number = next_in_line_part_numbers;

            line_index += 1;
        }
    }

    let sum = symbols.iter()
        .flat_map(|x| x)
        .filter(|x| x.number_of_attached_gears == 2).map(|x| x.gear_ratio)
        .sum::<i32>();
    println!("Finished calculating with sum: {}", sum);
}
