fn main() {
    let contents = include_str!("./input.txt");

    let mut instructions = vec![0];

    for line in contents.split("\n") {
        if line == "noop" {
            instructions.push(0);
            continue;
        }

        let mut split = line.split(" ");

        // This should be "addx" all the time and can discarded
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

    println!(
        "Day 10 part 1: {}",
        (vec![20, 60, 100, 140, 180, 220])
            .into_iter()
            .map(|i| get_register_value_at_index(i, &instructions) * i)
            .sum::<i32>()
    );

    let mut current_x = 1;
    let mut display: Vec<&str> = vec![];
    let mut cycle = 0;
    instructions.iter().for_each(|instruction| {
        current_x = current_x + instruction;

        if current_x.abs_diff(cycle % 40) <= 1 {
            display.push("#");
        } else {
            display.push(".");
        }

        cycle = cycle + 1;
    });

    println!("Day 10 part 2:");
    display
        .chunks(40)
        .for_each(|line| println!("{}", line.join("")));
}

fn get_register_value_at_index(index: i32, instructions: &Vec<i32>) -> i32 {
    let (read_until, _) = instructions.split_at(index.try_into().unwrap());

    read_until.iter().sum::<i32>() + 1
}
