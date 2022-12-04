use core::panic;
use std::{collections::HashMap, fs};

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    task_one(contents);
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

    println!("Priority sum: {:?}", priority_sum);
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
