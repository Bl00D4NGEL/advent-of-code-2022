use std::vec;

use regex::Regex;

fn main() {
    let contents = include_str!("./input.txt");

    let lines = contents.split("\n");

    let (move_lines, non_move_lines): (Vec<&str>, Vec<&str>) =
        lines.partition(|line| line.starts_with("move"));

    let mut cargo_stacks = non_move_lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    // Last line of cargo_stacks is describing the "indexes", everything above are the actual boxes
    cargo_stacks.pop();
    let stacks = parse_stacks(cargo_stacks);

    let moves = parse_moves(move_lines);

    let moved_stacks_9000 = crate_mover_9000(moves.clone(), &mut stacks.clone());
    let moved_stacks_9001 = crate_mover_9001(moves.clone(), &mut stacks.clone());

    let top_crates_9000 = get_top_crates(moved_stacks_9000);
    let top_crates_9001 = get_top_crates(moved_stacks_9001);
    println!("Part 1 top crates: {:?}", top_crates_9000);
    println!("Part 2 top crates: {:?}", top_crates_9001);
}

fn parse_stacks(cargo_stack_lines: Vec<&str>) -> Vec<Vec<char>> {
    // We are iterating over the lines and push them into new cargo "stacks"
    // Which are vectors that contain Option<char>. The Option is None if the stack entry is a space char, otherwise it's Some with the val in the brackets
    let parsed_stacks = cargo_stack_lines
        .into_iter()
        .map(|cs| {
            let mut parsed = vec![];
            for (i, char) in cs.chars().enumerate() {
                if i % 4 == 1 {
                    parsed.push(match char {
                        ' ' => Option::None,
                        v => Option::Some(v),
                    });
                }
            }

            parsed
        })
        .collect::<Vec<Vec<Option<char>>>>();

    let length = parsed_stacks.first().unwrap().len();

    let mut stacks = vec![];
    for _ in 0..=length {
        stacks.push(vec![]);
    }

    parsed_stacks.into_iter().for_each(|parsed_stack| {
        parsed_stack
            .into_iter()
            .enumerate()
            .for_each(|(idx, stack)| {
                if stack.is_some() {
                    stacks[idx + 1].insert(0, stack.unwrap());
                    // stacks[idx + 1].push(stack.unwrap());
                }
            });
    });

    stacks
}

/*
 * Returns a vector of tuples
 * The tuples contain the amount of cargos to move, where to get them from and where to put them
 */
fn parse_moves(move_lines: Vec<&str>) -> Vec<(i32, usize, usize)> {
    // move x from y to z
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)\s*$").unwrap();
    move_lines
        .into_iter()
        .map(|line| {
            let capture = re.captures(line);
            return match capture {
                Some(v) => (
                    v.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    v.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    v.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                ),
                None => panic!("Regex did not match anything"),
            };
        })
        .collect()
}

fn crate_mover_9000(
    moves: Vec<(i32, usize, usize)>,
    stacks: &mut Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    moves
        .into_iter()
        .for_each(|(amount, index_from, index_to)| {
            let mut taken = 0;
            while taken < amount {
                let crates_to_remove_from = stacks.get_mut(index_from).unwrap();
                let top_crate = crates_to_remove_from.pop().unwrap();

                let creates_to_push_to = stacks.get_mut(index_to).unwrap();
                creates_to_push_to.push(top_crate);
                taken = taken + 1;
            }
        });

    stacks.to_owned()
}

fn crate_mover_9001(
    moves: Vec<(i32, usize, usize)>,
    stacks: &mut Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    moves
        .into_iter()
        .for_each(|(amount, index_from, index_to)| {
            let crates_to_remove_from = stacks.get_mut(index_from).unwrap();
            let mut taken = 0;
            let mut top_crates = vec![];
            while taken < amount {
                let top_crate = crates_to_remove_from.pop().unwrap();

                top_crates.insert(0, top_crate);
                taken = taken + 1;
            }
            let creates_to_push_to = stacks.get_mut(index_to).unwrap();
            creates_to_push_to.append(&mut top_crates);
        });
    stacks.to_owned()
}

fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .filter(|stack| stack.len() != 0)
        .map(|stack| stack.last().unwrap())
        .collect()
}
