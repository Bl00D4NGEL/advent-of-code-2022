use core::panic;
use std::ops::RangeInclusive;

fn main() {
    let contents = include_str!("./input.txt");

    let lines = contents.split("\n");
    let mut full_overlaps = 0;
    let mut partial_overlaps = 0;
    for line in lines {
        let mut pairs = line.split(",");

        if pairs.clone().count() != 2 {
            panic!("Pairs must be of size 2");
        }

        let pair_one_range = get_range_from_string(pairs.next().unwrap());
        let pair_two_range = get_range_from_string(pairs.next().unwrap());

        if fully_overlaps(&pair_one_range, &pair_two_range) {
            full_overlaps = full_overlaps + 1;
        }
        if partially_overlaps(&pair_one_range, &pair_two_range) {
            partial_overlaps = partial_overlaps + 1;
        }
    }

    println!("Part 1 overlaps: {:?}", full_overlaps);
    println!("Part 2 overlaps: {:?}", partial_overlaps);
}

fn fully_overlaps(first: &RangeInclusive<i32>, second: &RangeInclusive<i32>) -> bool {
    if first.contains(&second.start()) && first.contains(&second.end()) {
        return true;
    }

    if second.contains(&first.start()) && second.contains(&first.end()) {
        return true;
    }

    return false;
}

fn partially_overlaps(first: &RangeInclusive<i32>, second: &RangeInclusive<i32>) -> bool {
    if first.contains(&second.start()) {
        return true;
    }

    if first.contains(&second.end()) {
        return true;
    }

    if second.contains(&first.start()) {
        return true;
    }

    return second.contains(&first.end());
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
