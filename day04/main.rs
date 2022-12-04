use core::panic;
use std::{
    fs,
    ops::{Range, RangeInclusive},
};

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    task_one(contents);
}

fn task_one(contents: String) {
    let lines = contents.split("\n");
    let mut overlaps = 0;
    for line in lines {
        let mut pairs = line.split(",");

        if pairs.clone().count() != 2 {
            panic!("Pairs must be of size 2");
        }

        let pair_one_range = get_range_from_string(pairs.next().unwrap());
        let pair_two_range = get_range_from_string(pairs.next().unwrap());

        // If the start and the end are contained in the other range the full range must be in it
        if pair_one_range.contains(&pair_two_range.start())
            && pair_one_range.contains(&pair_two_range.end())
        {
            overlaps = overlaps + 1;
        } else if pair_two_range.contains(&pair_one_range.start())
            && pair_two_range.contains(&pair_one_range.end())
        {
            overlaps = overlaps + 1;
        }
    }

    println!("Part 1 overlaps: {:?}", overlaps);
}

fn get_range_from_string(input: &str) -> RangeInclusive<i32> {
    let mut pair_one_ranges = input.split("-");
    if pair_one_ranges.clone().count() != 2 {
        panic!("Pair one range could not be split");
    }
    let range_start = pair_one_ranges.next().unwrap().parse::<i32>().unwrap();
    let range_end = pair_one_ranges.next().unwrap().parse::<i32>().unwrap();
    RangeInclusive::new(range_start, range_end)
}
