use std::io::{self, BufRead};


fn read_and_sort_box_ids() -> Vec<String> {
    let stdin = io::stdin();

    let mut boxes = stdin.lock().lines().map(|s| s.unwrap()).collect::<Vec<String>>();
    boxes.sort();
    
    return boxes;
}

fn find_similar_boxes(box_ids: Vec<String>) -> Option<String> {
    for pair in box_ids.windows(2) {
        if pair.is_empty() {
            return None;
        }

        let first_id = &pair[0];
        let second_id = &pair[1];

        let first_id_chars = first_id.chars();
        let second_id_chars = second_id.chars();
        let expected_count = first_id.len() - 1;

        let mut matching_chars: Vec<char> = Vec::new();

        for pair in first_id_chars.zip(second_id_chars) {
            let (first_char, second_char) = pair;

            if first_char == second_char {
                matching_chars.push(first_char);
            }

        }

        if matching_chars.len() == expected_count {
            return Some(matching_chars.into_iter().collect());
        }
    }

    return None;
}


fn main() {
    let box_ids = read_and_sort_box_ids();

    let similar = find_similar_boxes(box_ids);

    println!("{:?}", similar.unwrap());
}