use std::path::Path;
use shared_code::get_file_lines;
use shared_code::part_number::{PartNumber, read_parts_in_line, read_parts_in_line_day_1, Symbol};

fn main() {
    let mut last_line_symbols: Vec<Symbol> = vec![];
    let mut last_line_part_number: Vec<PartNumber> = vec![];
    let mut valid_part_numbers: Vec<PartNumber> = vec![];
    let mut line_index = 0;
    for line in get_file_lines(Path::new("E:/workspaces/advent-of-code-2023/advent-of-code-3/input.txt")) {
        let mut next_in_line_symbols: Vec<Symbol> = vec![];
        let mut next_in_line_part_numbers : Vec<PartNumber> = vec![];
        if let Ok(ok_line) = line {
            println!("Processing line: \n{}", ok_line);
            println!("line index {}", line_index);

            for part in &mut read_parts_in_line_day_1(ok_line, line_index) {
                if part.is_valid {
                    println!("adding: {}", part.get_integer_value());
                    valid_part_numbers.push(part.clone());
                    continue;
                } else if part.is_symbol {
                    if line_index > 0 {
                        for last_line_part in &mut last_line_part_number {
                            if last_line_part.is_valid {
                                println!("Skipping last line: is_valid {}, is_symbol {}", last_line_part.is_valid, last_line_part.is_symbol);
                                continue;
                            }

                            if last_line_part.index_span.contains(&(part.index_span.first().unwrap() - 1)) ||
                                last_line_part.index_span.contains(&(part.index_span.first().unwrap())) ||
                                last_line_part.index_span.contains(&(part.index_span.first().unwrap() + 1)) {
                                print!("marking part as valid: symbol {}, valid {}, with span ", last_line_part.is_symbol, last_line_part.is_valid);
                                for span in last_line_part.index_span.iter() {
                                    print!("{} ", span);
                                }

                                println!("");
                                last_line_part.to_owned().mark_as_valid();

                                println!("symbol span: {}", part.index_span.first().unwrap());
                                println!("adding because last line number: {} with symbol {}", last_line_part.get_integer_value(), part.string_value);
                                valid_part_numbers.push(last_line_part.clone());
                            }
                        }
                    }

                    next_in_line_symbols.push(Symbol { index: *part.index_span.first().unwrap() as i32, symbol: part.string_value});
                } else {
                    if line_index != 0 {
                        let last_line_valid = last_line_symbols
                            .iter()
                            .find(|&x| part.index_span.contains(&(0.max(x.index - 1) as usize)) || part.index_span.contains(&(x.index as usize)) || part.index_span.contains(&(139.min((x.index + 1) as usize))));

                        if last_line_valid.is_some() {
                            println!("------");
                            println!("symbol span: {}", last_line_valid.unwrap().index);
                            print!("adding {} because of previous line symbol: {}, with span ", part.get_integer_value(), last_line_valid.unwrap().symbol);
                            for span in part.index_span.iter() {
                                print!("{} ", span)
                            }

                            println!("");
                            println!("-------");

                            valid_part_numbers.push(part.clone());
                            continue;
                        }
                    }

                    next_in_line_part_numbers.push(part);
                }
            }
            last_line_symbols = next_in_line_symbols;
            last_line_part_number = next_in_line_part_numbers;

            line_index += 1;
        }
    }

    let sum = valid_part_numbers.iter().map(|x| x.get_integer_value()).sum::<i32>();
    println!("Finished calculating with sum: {}", sum);
}
