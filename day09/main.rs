use std::collections::LinkedList;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn is_adjacent_to(&self, other_position: &Position) -> bool {
        // Same position = adjacent
        if other_position.x == self.x && other_position.y == self.y {
            return true;
        }

        // If the difference to x or y is greater than 1 it cant be adjacent
        if self.x.abs_diff(other_position.x) > 1 {
            return false;
        }

        self.y.abs_diff(other_position.y) <= 1
    }

    pub fn close_gap_to(&mut self, other_position: &Position) {
        let x_diff = self.x - other_position.x;
        let y_diff = self.y - other_position.y;

        let x_increment = match x_diff {
            -2 => 1,
            -1 => 1,
            1 => -1,
            2 => -1,
            _ => 0,
        };

        let y_increment = match y_diff {
            -2 => 1,
            -1 => 1,
            1 => -1,
            2 => -1,
            _ => 0,
        };

        self.x = self.x + x_increment;
        self.y = self.y + y_increment;
    }

    pub fn move_up(&mut self) {
        self.x = self.x + 1
    }

    pub fn move_down(&mut self) {
        self.x = self.x - 1
    }

    pub fn move_left(&mut self) {
        self.y = self.y - 1
    }

    pub fn move_right(&mut self) {
        self.y = self.y + 1
    }
}

fn main() {
    let contents = include_str!("./input.txt");

    let directions = parse_directions(contents);
    part_1(&directions);
    part_2(&directions);
}

fn parse_directions(contents: &str) -> Vec<Direction> {
    let directions = contents
        .split("\n")
        .map(|line| {
            let mut splitted_line = line.split(" ");
            let direction = match splitted_line.next() {
                None => panic!("Cannot parse direction from {}", line),
                Some(v) => match v {
                    "U" => Direction::UP,
                    "D" => Direction::DOWN,
                    "R" => Direction::RIGHT,
                    "L" => Direction::LEFT,
                    _ => panic!("Cannot parse direction {}", v),
                },
            };
            let amount = match splitted_line.next() {
                None => panic!("Coult not parse amount from {}", line),
                Some(v) => v.parse::<usize>().unwrap(),
            };

            vec![direction].repeat(amount)
        })
        .flatten()
        .collect::<Vec<Direction>>();
    directions
}

fn part_1(directions: &Vec<Direction>) {
    let mut tail_position = Position::new();
    let mut head_position = Position::new();

    let mut visited = vec![(tail_position.x, tail_position.y)];

    for direction in directions {
        move_position_based_on_direction(direction, &mut head_position);

        if !tail_position.is_adjacent_to(&head_position) {
            tail_position.close_gap_to(&head_position);
            visited.push((tail_position.x, tail_position.y));
        }
    }

    visited.sort();
    visited.dedup();

    println!("Day 9 part 1: {}", visited.len());
}

fn part_2(directions: &Vec<Direction>) {
    let mut head_position = Position::new();

    let mut visited = vec![];

    let mut knot_chain = LinkedList::new();
    for _ in 0..9 {
        knot_chain.push_back(Position::new());
    }

    for direction in directions {
        move_position_based_on_direction(direction, &mut head_position);

        move_knot_chain_to(&mut knot_chain, &head_position);

        let last_knot = knot_chain.back().unwrap();
        visited.push((last_knot.x, last_knot.y));
    }

    visited.sort();
    visited.dedup();
    println!("Day 9 part 2: {}", visited.len());
}

fn move_position_based_on_direction(direction: &Direction, position: &mut Position) {
    match direction {
        Direction::DOWN => position.move_down(),
        Direction::UP => position.move_up(),
        Direction::LEFT => position.move_left(),
        Direction::RIGHT => position.move_right(),
    }
}

fn move_knot_chain_to(knot_chain: &mut LinkedList<Position>, position: &Position) {
    let mut knot_iter = knot_chain.iter_mut();

    let mut prev = knot_iter
        .next()
        .expect("Knot chain must have at least one element");
    if !prev.is_adjacent_to(position) {
        prev.close_gap_to(position);
    }

    for knot in knot_iter {
        if knot.is_adjacent_to(&prev) {
            break;
        }

        knot.close_gap_to(&prev);

        prev = knot;
    }
}
