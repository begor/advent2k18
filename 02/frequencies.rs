use std::io::{self, BufRead};
use std::collections::HashSet;


fn read_freqs() -> Vec<String> {
    let stdin = io::stdin();

    return stdin.lock().lines().map(|s| s.unwrap()).collect();
}


fn parse_freq(freq: &String) -> i32 {
    return match freq.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
}


fn find_repeating(freqs: Vec<String>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();

    let mut current: i32 = 0;

    set.insert(current);

    for freq in freqs.iter().cycle() {
        current += parse_freq(freq);

        if set.contains(&current) {
            break;
        } else {
            set.insert(current);
        }
    }

    return current;
}


fn main() {
    let freqs = read_freqs();
    let result = find_repeating(freqs);

    println!("{}", result);
}