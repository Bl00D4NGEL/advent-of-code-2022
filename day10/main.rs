fn main() {
    let contents = include_str!("./input.txt");

    let mut instructions = vec![0];

    for line in contents.split("\n") {
        if line == "noop" {
            instructions.push(0);
            continue;
        }

        let mut split = line.split(" ");
        split.next();
        let amount = match split.next() {
            None => 0,
            Some(v) => match v.parse::<i32>() {
                Ok(v) => v,
                Err(err) => panic!("Could not parse addx val ({}): {:?}", v, err),
            },
        };

        instructions.push(0);
        instructions.push(amount);
    }

    let requested_indexes: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    let mapped = requested_indexes
        .into_iter()
        .map(|i| get_register_value_at_index(i, &instructions) * i)
        .collect::<Vec<i32>>();

    println!("Day 10 part 1: {}", mapped.iter().sum::<i32>());
}

fn get_register_value_at_index(index: i32, instructions: &Vec<i32>) -> i32 {
    let (read_until, _) = instructions.split_at(index.try_into().unwrap());

    read_until.iter().sum::<i32>() + 1
}
