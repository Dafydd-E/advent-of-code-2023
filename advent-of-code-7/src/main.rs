mod card;
mod hand;
mod hand_type;
mod handing;
mod card_type;
mod hand_with_bid;

use std::collections::{BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::time::Instant;
use crate::hand_with_bid::HandWithBid;

fn main() {
    let now = Instant::now();
    let ref mut hands: BTreeSet<HandWithBid> = BTreeSet::new();
    let lines = get_file_lines(Path::new("E:/workspaces/advent-of-code-2023/advent-of-code-7/src/input.txt"));
    for line in lines {
        if let Ok(ok_line) = line {
            let mut hand = HandWithBid::parse(ok_line);
            hand.hand.get_hand_type_mut();
            hand.hand.get_high_card();
            hands.insert(hand);
        }
    }

    let mut sum = 0;
    for (index, hand) in hands.iter().enumerate() {
        let rank = index + 1;
        sum += rank * hand.bid;
    }
    println!("{}", sum);
    println!("execution time {}ms", now.elapsed().as_millis());
}

pub fn get_file_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Ok(ok_file) = file {
        let reader = BufReader::new(ok_file);
        return reader.lines();
    }

    panic!("File not available");
}
