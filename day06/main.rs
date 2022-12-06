use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut lines = contents.split("\n");
    let line = lines.next().unwrap();
    let first_unique_sequence_index = find_first_occurance_of_unique_sequence(line);
    println!("Day 6 Part 1: {:?}", first_unique_sequence_index + 1);
}

fn find_first_occurance_of_unique_sequence(input: &str) -> usize {
    let mut chars = vec![];
    for (idx, char) in input.chars().enumerate() {
        chars.insert(0, char);
        if chars.len() == 4 {
            // Check if chars is vec of 4 different chars
            let mut chars_to_check = chars.clone();
            chars_to_check.sort();
            chars_to_check.dedup();
            if chars_to_check.len() == 4 {
                return idx;
            }
            chars.pop();
        }
    }

    panic!("Could not find unique sequence in input!");
}
