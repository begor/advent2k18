use std::io::{self, BufRead};
use std::collections::HashMap;  // Because Rust doesn't have Counter in stdlib, only in crate


fn read_box_ids() -> Vec<String> {
    let stdin = io::stdin();

    return stdin.lock().lines().map(|s| s.unwrap()).collect();
}


fn calculate_checksum(box_ids: Vec<String>) -> i32 {
    let mut two_letters = 0;
    let mut three_letters = 0;

    for id in box_ids {
        let mut counter: HashMap<char, i32> = HashMap::new();

        let mut has_two = false;
        let mut has_three = false;

        // No Counter for you, sorry
        for char in id.chars() {
            *counter.entry(char).or_insert(0) += 1;
        }

        for (_, count) in counter {
            match count {
                2 => has_two = true,
                3 => has_three = true,
                _ => {}
            }
        }

        two_letters += has_two as i32;
        three_letters += has_three as i32;
    }

    return two_letters * three_letters;
}


fn main() {
    let box_ids = read_box_ids();

    let checksum = calculate_checksum(box_ids);

    println!("{}", checksum);
}