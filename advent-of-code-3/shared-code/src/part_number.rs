use std::ops::Index;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PartNumber {
    pub line_index: i32,
    pub index_span: Vec<usize>,
    pub is_valid: bool,
    pub is_symbol: bool,
    pub string_value: String,
    pub number_of_attached_gears: i32,
    pub gear_ratio: i32
}

impl PartNumber {
    pub fn new(line_index: i32, index_span: Vec<usize>, string_value: String, is_symbol: bool, number_of_attached_gears: i32, gear_ratio: i32) -> Self {
        Self { line_index, is_symbol, index_span, string_value, is_valid: false, number_of_attached_gears, gear_ratio}
    }

    pub fn new_with_valid(line_index: i32, index_span: Vec<usize>, string_value: String, is_symbol: bool, is_valid: bool) -> Self {
        Self { line_index, is_symbol, index_span, string_value, is_valid, number_of_attached_gears: 0, gear_ratio: 1 }
    }

    pub fn get_integer_value(&self) -> i32 {
        return self.string_value.parse::<i32>().unwrap();
    }

    pub fn add_character(&mut self, character: char) {
        self.string_value = format!("{}{}", self.string_value, character);
    }

    pub fn is_gear(&self) -> bool {
        return self.string_value == GEAR.to_string();
    }

    pub fn mark_as_valid(&mut self) {
        self.is_valid = true;
    }
}

impl Default for PartNumber {
    fn default() -> Self {
        return PartNumber::new(-1, vec![], String::default(), false, -1, 1);
    }
}

pub struct Symbol {
    pub symbol: String,
    pub index: i32
}

impl Symbol {

}

impl Default for Symbol {
    fn default() -> Self {
        Self { index: -1, symbol: String::default() }
    }
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"[^\sa-zA-Z\.]").unwrap();
    static ref IS_SYMBOL: Regex = Regex::new(r"[^\sa-zA-Z\.\d]").unwrap();
    static ref GEAR: String = String::from("*");
}

const RADIX: u32 = 10;

pub fn read_parts_in_line(line: String, line_index: i32) -> impl Iterator<Item=PartNumber> {
    let mut part_number = PartNumber::default();
    println!("length {}", line.len());
    let mut input_vector = vec![];
    let mut previous_char_was_symbol = false;
    for (i, char) in line.chars().enumerate() {
        if IS_SYMBOL.is_match(char.to_string().as_str()) {
            let mut gears = 0;
            let mut gear_ratio = 1;
            if part_number.line_index != -1 {
                // we have a numerical character to push to the vector.
                part_number.is_valid = true;

                // symbol under char has at least 1 attached to gear.
                gears += 1;

                input_vector.push(part_number.clone());
                gear_ratio *= part_number.clone().get_integer_value();
                part_number = PartNumber::default();
            }

            if char.to_string() != GEAR.to_string() {
                gears = -1;
            }
            previous_char_was_symbol = true;
            // horizontal gear ratios, could have two horizontally!!
            // send symbol to the vector array too.
            input_vector.push(PartNumber::new(line_index, vec![i], char.to_string(), true, gears, gear_ratio));
        } else if char.is_digit(RADIX) {
            if part_number.line_index == -1 {
                part_number = PartNumber::new_with_valid(line_index, vec![i], char.to_string(), false, previous_char_was_symbol.clone());
            } else {
                part_number.index_span.push(i);
                part_number.add_character(char);
            }

            previous_char_was_symbol = false;
        } else if part_number.line_index != -1 {
            // we have finished processing a symbol or a number and there is a machine
            // part that can be pushed to the vector,

            if i > 0 {
                let last_symbol = input_vector.last_mut();
                if last_symbol.is_some() {
                    let last = last_symbol.unwrap();
                    if last.is_gear() && (last.index_span.last().unwrap() + 1 == *(part_number.index_span.first().unwrap())) {
                        last.number_of_attached_gears += 1;
                        last.gear_ratio *= part_number.get_integer_value();
                    }
                }
            }

            input_vector.push(part_number.clone());
            part_number = PartNumber::default();

            previous_char_was_symbol = false;
        } else {
            previous_char_was_symbol = false;
        }
    }

    if part_number.line_index != -1 {
        input_vector.push(part_number.clone());
        part_number = PartNumber::default();
    }

    return input_vector.into_iter();
}

pub fn read_parts_in_line_day_1(line: String, line_index: i32) -> impl Iterator<Item=PartNumber> {
    let mut part_number = PartNumber::default();
    println!("length {}", line.len());
    let mut input_vector = vec![];
    let mut previous_char_was_symbol = false;
    for (i, char) in line.chars().enumerate() {
        if IS_SYMBOL.is_match(char.to_string().as_str()) {
            let mut gears = 0;
            let mut gear_ratio = 1;
            if part_number.line_index != -1 {
                // we have a numerical character to push to the vector.
                part_number.is_valid = true;

                // symbol under char has at least 1 attached to gear.
                gears += 1;

                input_vector.push(part_number.clone());
                gear_ratio *= part_number.clone().get_integer_value();
                part_number = PartNumber::default();
            }

            let forward_char = peak_forward(line.clone(), i + 1);
            if forward_char.is_digit(RADIX) {
                gears += 1;
            }

            if char.to_string() != GEAR.to_string() {
                gears = -1;
            }
            previous_char_was_symbol = true;
            // horizontal gear ratios, could have two horizontally!!
            // send symbol to the vector array too.
            input_vector.push(PartNumber::new(line_index, vec![i], char.to_string(), true, gears, gear_ratio));
        } else if char.is_digit(RADIX) {
            if part_number.line_index == -1 {
                part_number = PartNumber::new_with_valid(line_index, vec![i], char.to_string(), false, previous_char_was_symbol.clone());
            } else {
                part_number.index_span.push(i);
                part_number.add_character(char);
            }

            previous_char_was_symbol = false;
        } else if part_number.line_index != -1 {
            if i > 0 {
                let last_symbol = input_vector.last_mut();
                if last_symbol.is_some() {
                    let last = last_symbol.unwrap();
                    if last.is_gear() {
                        last.number_of_attached_gears += 1;
                        last.gear_ratio *= part_number.get_integer_value();
                    }
                }
            }

            input_vector.push(part_number.clone());
            part_number = PartNumber::default();

            previous_char_was_symbol = false;
        } else {
            previous_char_was_symbol = false;
        }
    }

    if part_number.line_index != -1 {
        input_vector.push(part_number.clone());
        part_number = PartNumber::default();
    }
    return input_vector.into_iter();
}

fn peak_forward(line: String, peak_index: usize) -> char {
    if peak_index >= 140 {
        return char::default();
    }

    let some_char = line.chars().nth(peak_index).unwrap();
    return some_char;
}