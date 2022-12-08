fn main() {
    let contents = include_str!("./input.txt");

    let mut lines = contents.split("\n");
    let line = lines.next().unwrap();
    let start_of_packet_index = find_first_occurance_of_unique_sequence(line, 4);
    let start_of_message_index = find_first_occurance_of_unique_sequence(line, 14);
    println!("Day 6 Part 1: {:?}", start_of_packet_index + 1);
    println!("Day 6 Part 2: {:?}", start_of_message_index + 1);
}

fn find_first_occurance_of_unique_sequence(input: &str, unique_sequence_size: usize) -> usize {
    let mut chars = vec![];
    for (idx, char) in input.chars().enumerate() {
        chars.insert(0, char);
        if chars.len() == unique_sequence_size {
            // Check if chars is vec of uniqu,e_sequence_size different chars
            let mut chars_to_check = chars.clone();
            chars_to_check.sort();
            chars_to_check.dedup();
            if chars_to_check.len() == unique_sequence_size {
                return idx;
            }
            chars.pop();
        }
    }

    panic!("Could not find unique sequence in input!");
}
