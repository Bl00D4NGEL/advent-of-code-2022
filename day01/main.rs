use std::{fs, vec};

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Split file by new line
    let lines = contents.split("\n");

    // Group entries (empty line = new group)
    let mut elf_cargo: Vec<Vec<i32>> = vec![];
    let mut group: Vec<i32> = vec![];
    for line in lines {
        if line.is_empty() {
            elf_cargo.push(group.clone());
            group = vec![];
            continue;
        }

        group.push(line.parse().expect("Input should be integer values only"));
    }

    // Calculate sum of group
    let elf_cargo_sums: Vec<i32> = elf_cargo.iter().map(|cargo| cargo.iter().sum()).collect();

    // Get biggest cargo sum
    let mut biggest_cargo = 0;
    for cargo_sum in elf_cargo_sums {
        if biggest_cargo < cargo_sum {
            biggest_cargo = cargo_sum;
        }
    }

    println!("Biggest cargo: {:?}", biggest_cargo);
}
