#[derive(Clone, Copy, Debug)]
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
        // position is only above/below
        if y_diff == 0 {
            if x_diff == 2 {
                self.x = self.x - 1;
            }
            if x_diff == -2 {
                self.x = self.x + 1;
            }
        }

        if y_diff == 1 {
            if x_diff == 2 {
                self.x = self.x - 1;
                self.y = self.y - 1;
            }
            if x_diff == -2 {
                self.x = self.x + 1;
                self.y = self.y - 1;
            }
        }

        if y_diff == -1 {
            if x_diff == 2 {
                self.x = self.x - 1;
                self.y = self.y + 1;
            }
            if x_diff == -2 {
                self.x = self.x + 1;
                self.y = self.y + 1;
            }
        }

        // position is only left/right
        if x_diff == 0 {
            if y_diff == 2 {
                self.y = self.y - 1;
            }
            if y_diff == -2 {
                self.y = self.y + 1;
            }
        }

        if x_diff == 1 {
            if y_diff == 2 {
                self.x = self.x - 1;
                self.y = self.y - 1;
            }
            if y_diff == -2 {
                self.x = self.x - 1;
                self.y = self.y + 1;
            }
        }
        if x_diff == -1 {
            if y_diff == 2 {
                self.x = self.x + 1;
                self.y = self.y - 1;
            }
            if y_diff == -2 {
                self.x = self.x + 1;
                self.y = self.y + 1;
            }
        }
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

    let mut tail_position = Position::new();
    let mut head_position = Position::new();

    let mut visited = vec![(tail_position.x, tail_position.y)];

    for direction in &directions {
        match direction {
            Direction::DOWN => head_position.move_down(),
            Direction::UP => head_position.move_up(),
            Direction::LEFT => head_position.move_left(),
            Direction::RIGHT => head_position.move_right(),
        }

        if !tail_position.is_adjacent_to(&head_position) {
            tail_position.close_gap_to(&head_position);
            visited.push((tail_position.x, tail_position.y));
        }
    }

    visited.sort();
    visited.dedup();

    println!("Day 9 part 1: {}", visited.len());
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
