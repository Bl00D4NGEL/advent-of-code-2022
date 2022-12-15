use std::vec;

fn main() {
    let contents = include_str!("./input.txt");

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
    let mut elf_cargo_sums: Vec<i32> = elf_cargo.iter().map(|cargo| cargo.iter().sum()).collect();

    // Sort list so the biggest sum is the last element
    elf_cargo_sums.sort();

    println!("Biggest cargo: {:?}", elf_cargo_sums.last());

    // Part two requires the sum of the three biggest cargos
    let top_three_cargo_sums: i32 = elf_cargo_sums.iter().rev().take(3).sum();

    println!("Top 3 cargo sums: {:?}", top_three_cargo_sums);
}
