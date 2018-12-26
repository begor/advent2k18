extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;

use regex::Regex;


#[derive(Debug)]
enum DutyEvent {
    Start,
    WakeUp,
    Asleep
}


#[derive(Debug)]
struct DutyRecord {
    date: String,
    minute: String,
    event: DutyEvent,
}


fn read_duty_records() -> Vec<String> {
    let stdin = io::stdin();

    let mut records = stdin.lock().lines().map(|s| s.unwrap()).collect::<Vec<String>>();
    records.sort();
    
    return records;
}


fn parse_record_string(record_string: &String) -> &String {
    let re = Regex::new(r"^\[(.*)\] (.*)$").unwrap();

    for cap in re.captures_iter(record_string) {
        println!("{:?}", cap);
    }

    return record_string;
}


fn main() {
    let records_strings = read_duty_records();

    records_strings.iter().map(parse_record_string).count();
}