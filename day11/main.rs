use std::{
    collections::HashMap,
    ops::{Div, Mul},
};

fn main() {
    let contents = include_str!("./input.txt");

    let lines = contents
        .split("\n")
        .into_iter()
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    let monkey_inputs = lines
        .chunks(7)
        .into_iter()
        .map(|c| {
            c.into_iter()
                .filter(|l| !l.is_empty())
                .collect::<Vec<&&str>>()
        })
        .collect::<Vec<Vec<&&str>>>();

    let mut items_by_monkey = HashMap::new();
    for (idx, input) in monkey_inputs.iter().enumerate() {
        let items = match create_items(&input) {
            None => continue,
            Some(v) => v,
        };
        items_by_monkey.insert(idx, items);
    }

    let mut total_handled_items = vec![];
    for _ in 0..20 {
        let handled_items = play_round(&mut items_by_monkey, &monkey_inputs);
        total_handled_items.push(handled_items);
    }
    let monkey_ids = items_by_monkey
        .keys()
        .into_iter()
        .map(|k| k.to_owned())
        .collect::<Vec<usize>>();

    let mut handle_sums = vec![];

    for monkey_id in monkey_ids {
        handle_sums.push(
            total_handled_items
                .iter()
                .map(|hashmap| hashmap.get(&monkey_id).unwrap())
                .sum::<usize>(),
        );
    }
    // Sort sums and reverse that highest handled is first
    handle_sums.sort();
    handle_sums.reverse();

    let highest_sums_product = handle_sums
        .into_iter()
        .take(2)
        .into_iter()
        .reduce(|acc, val| acc.mul(val))
        .unwrap();

    println!("Day 11 part 1: {}", highest_sums_product);
}

fn create_items<'a>(input: &'a Vec<&&str>) -> Option<Vec<i32>> {
    Some(
        input
            .get(1)?
            .split_at("Starting items: ".len())
            .1
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>(),
    )
}

fn play_round<'a>(
    items_by_monkey: &'a mut HashMap<usize, Vec<i32>>,
    monkey_inputs: &Vec<Vec<&&str>>,
) -> HashMap<usize, usize> {
    let mut monkey_ids = items_by_monkey
        .keys()
        .into_iter()
        .map(|k| k.to_owned())
        .collect::<Vec<usize>>();
    monkey_ids.sort();

    let mut items_handled = HashMap::new();

    for monkey_id in monkey_ids {
        let monkey_input = monkey_inputs.get(monkey_id).unwrap();

        let items = match items_by_monkey.get_mut(&monkey_id) {
            None => break,
            Some(v) => v,
        };
        items_handled.insert(monkey_id, items.len());
        let mut new_item_locations: Vec<(usize, i32)> = vec![];
        loop {
            let item = match items.pop() {
                None => break,
                Some(v) => v,
            };

            // Figure out monkey specific action
            let mut inspected_item = inspect_item_for_monkey(item, monkey_input);
            // Monkey gets bored
            inspected_item = inspected_item.div(3);

            let monkey_divisor = match get_monkey_divisor(monkey_input) {
                None => continue,
                Some(v) => v,
            };

            let (positive_target, negative_target) = match get_monkey_locations(monkey_input) {
                None => continue,
                Some(v) => v,
            };

            if inspected_item % monkey_divisor == 0 {
                new_item_locations.push((positive_target, inspected_item));
            } else {
                new_item_locations.push((negative_target, inspected_item));
            }
        }

        for (idx, item) in new_item_locations {
            items_by_monkey.get_mut(&idx).unwrap().push(item);
        }
    }

    items_handled
}

fn inspect_item_for_monkey<'a>(item: i32, input: &'a Vec<&&str>) -> i32 {
    let operation = match input.get(2) {
        None => return item,
        Some(v) => v.split_at("Operation: new = old ".len()).1,
    };

    if operation == "* old" {
        return item * item;
    }

    let (operand, value) = operation.split_at(1);
    let trimmed_value = value.trim();

    if operand == "*" {
        return item * trimmed_value.parse::<i32>().unwrap_or(1);
    }
    if operand == "+" {
        return item + trimmed_value.parse::<i32>().unwrap_or(0);
    }

    return item;
}

fn get_monkey_divisor(input: &Vec<&&str>) -> Option<i32> {
    Some(
        input
            .get(3)?
            .split_at("Test: divisble by ".len())
            .1
            .trim()
            .parse::<i32>()
            .unwrap(),
    )
}

fn get_monkey_locations(input: &Vec<&&str>) -> Option<(usize, usize)> {
    let positive_test_target_index = input
        .get(4)?
        .split_at("If true: throw to monkey ".len())
        .1
        .trim()
        .parse::<usize>()
        .unwrap();

    let negative_test_target_index = input
        .get(5)?
        .split_at("If false: throw to monkey ".len())
        .1
        .trim()
        .parse::<usize>()
        .unwrap();

    Some((positive_test_target_index, negative_test_target_index))
}
