use core::panic;
use std::{collections::HashMap, fs};

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    task_one(contents.clone());
    task_two(contents);
}

fn task_one(contents: String) {
    let lines = contents.split("\n");
    let alphabet_map = create_priority_map();

    let mut priority_sum = 0;

    for line in lines {
        if line.len() % 2 == 1 {
            panic!("Line length must be mod 2 == 0");
        }
        let (first, second) = line.split_at(line.len() / 2);
        let mut repeated = vec![];
        for item in second.chars() {
            if first.contains(item) {
                repeated.push(item);
            }
        }

        repeated.dedup();

        for repeat in repeated {
            let priority = alphabet_map
                .get(&repeat)
                .expect("Char must in alphabet map");

            priority_sum = priority_sum + priority;
        }
    }

    println!("Part 1: Priority sum: {:?}", priority_sum);
}

fn task_two(contents: String) {
    let lines = contents.split("\n");
    let alphabet_map = create_priority_map();

    let mut groups = vec![];
    let mut temp_group = vec![];
    for line in lines {
        temp_group.push(line);
        if temp_group.len() == 3 {
            groups.push(temp_group);

            temp_group = vec![];
        }
    }

    let mut priority_sum = 0;
    for group in groups {
        let mut backpack_hashmap = HashMap::new();
        for line in group {
            let mut chars = vec![];
            // Split str into chars
            // Put all chars into vec
            line.chars().for_each(|c| chars.push(c));

            // sort and dedup vec to remove duplicated chars
            chars.sort();
            chars.dedup();

            // put deduped values into hash map
            for deduped_char in chars {
                backpack_hashmap.insert(
                    deduped_char,
                    match backpack_hashmap.get(&deduped_char) {
                        None => 1,
                        Some(val) => val + 1,
                    },
                );
            }
        }

        // Find  which key in hashmap has value of 3 and add value to priority sum
        for (key, value) in backpack_hashmap.into_iter() {
            if value != 3 {
                continue;
            }
            let priority = alphabet_map.get(&key).expect("Char must in alphabet map");

            priority_sum = priority_sum + priority;
        }
    }

    println!("Part 2: Priority sum: {:?}", priority_sum);
}

fn create_priority_map() -> HashMap<char, i32> {
    let mut alphabet_map = HashMap::new();
    let mut iter = 1;
    ('a'..='z').into_iter().for_each(|letter| {
        alphabet_map.insert(letter, iter);
        iter = iter + 1;
    });

    ('A'..='Z').into_iter().for_each(|letter| {
        alphabet_map.insert(letter, iter);
        iter = iter + 1;
    });

    alphabet_map
}
