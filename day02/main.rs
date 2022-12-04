use core::panic;
use std::fs;

#[derive(PartialEq)]
enum GameResult {
    WIN,
    LOSE,
    DRAW,
}

#[derive(PartialEq)]
enum PossibleMoves {
    ROCK,
    PAPER,
    SCISSORS,
}

impl PossibleMoves {
    fn play_against(&self, move_to_play_against: &PossibleMoves) -> GameResult {
        match self {
            Self::ROCK => match move_to_play_against {
                Self::ROCK => GameResult::DRAW,
                Self::PAPER => GameResult::LOSE,
                Self::SCISSORS => GameResult::WIN,
            },

            Self::SCISSORS => match move_to_play_against {
                Self::ROCK => GameResult::LOSE,
                Self::PAPER => GameResult::WIN,
                Self::SCISSORS => GameResult::DRAW,
            },
            Self::PAPER => match move_to_play_against {
                Self::ROCK => GameResult::WIN,
                Self::PAPER => GameResult::DRAW,
                Self::SCISSORS => GameResult::LOSE,
            },
        }
    }
}

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    part_one(contents.clone());
    part_two(contents);
}

fn part_one(contents: String) {
    // Split file by new line
    let lines = contents.split("\n");

    let mut sum_points = 0;

    for line in lines {
        let mut moves = line.split(" ");

        let enemy_move = match moves.next() {
            None => panic!("Enemy move must exist"),
            Some("A") => PossibleMoves::ROCK,
            Some("B") => PossibleMoves::PAPER,
            Some("C") => PossibleMoves::SCISSORS,
            _ => panic!("Cannot match move"),
        };

        let my_move = match moves.next() {
            None => panic!("My move must exist"),
            Some("X") => PossibleMoves::ROCK,
            Some("Y") => PossibleMoves::PAPER,
            Some("Z") => PossibleMoves::SCISSORS,
            _ => panic!("Cannot match move"),
        };

        let move_based_points = match &my_move {
            PossibleMoves::ROCK => 1,
            PossibleMoves::PAPER => 2,
            PossibleMoves::SCISSORS => 3,
        };

        let result_based_points = match my_move.play_against(&enemy_move) {
            GameResult::LOSE => 0,
            GameResult::DRAW => 3,
            GameResult::WIN => 6,
        };

        sum_points = sum_points + move_based_points + result_based_points;
    }

    println!("Part 1: Sum points: {:?}", sum_points);
}

fn part_two(contents: String) {
    // Split file by new line
    let lines = contents.split("\n");

    let mut sum_points = 0;

    for line in lines {
        let mut moves = line.split(" ");

        let enemy_move = match moves.next() {
            None => panic!("Enemy move must exist"),
            Some("A") => PossibleMoves::ROCK,
            Some("B") => PossibleMoves::PAPER,
            Some("C") => PossibleMoves::SCISSORS,
            _ => panic!("Cannot match move"),
        };

        let expected_result = match &moves.next() {
            None => panic!("My move must exist"),
            Some("X") => GameResult::LOSE,
            Some("Y") => GameResult::DRAW,
            Some("Z") => GameResult::WIN,
            _ => panic!("Cannot match move"),
        };

        let move_for_result;

        if PossibleMoves::ROCK.play_against(&enemy_move) == expected_result {
            move_for_result = PossibleMoves::ROCK;
        } else if PossibleMoves::PAPER.play_against(&enemy_move) == expected_result {
            move_for_result = PossibleMoves::PAPER;
        } else {
            move_for_result = PossibleMoves::SCISSORS;
        }

        let move_based_points = match &move_for_result {
            PossibleMoves::ROCK => 1,
            PossibleMoves::PAPER => 2,
            PossibleMoves::SCISSORS => 3,
        };

        let result_based_points = match expected_result {
            GameResult::LOSE => 0,
            GameResult::DRAW => 3,
            GameResult::WIN => 6,
        };

        sum_points = sum_points + move_based_points + result_based_points;
    }

    println!("Part 2: Sum points: {:?}", sum_points);
}
