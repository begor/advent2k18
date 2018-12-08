use std::io::{self, BufRead};


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


fn calculate_result(freqs: Vec<String>) -> i32 {
    return freqs.iter().map(|f| parse_freq(f)).sum();
}


fn main() {
    let freqs = read_freqs();
    let result = calculate_result(freqs);

    println!("{}", result);
}