use std::fs;

use crate::solutions::day02::Move::{Paper, Rock, Scissors};
use crate::solutions::day02::Result::{Draw, Lose, Win};

pub fn part1() -> u32 {
    get_input_data("input.txt")
        .lines()
        .map(|line| {
            let mut splits = line.split(" ");
            (splits.next().unwrap(), splits.next().unwrap())
        })
        .map(|move_pair| (convert_to_move(move_pair.0), convert_to_move(move_pair.1)))
        .map(|move_pair| move_pair.1.beats(&move_pair.0).value() + move_pair.1.value())
        .sum()
}

pub fn part2() -> u32 {

    get_input_data("input.txt")
        .lines()
        .map(|line| {
            let mut splits = line.split(" ");
            (splits.next().unwrap(), splits.next().unwrap())
        })
        .map(|move_result| (convert_to_move(move_result.0), convert_to_result(move_result.1)))
        .map(|move_result| (move_result.0.result_obtained_by(&move_result.1), move_result.1))
        .map(|a| a.0.value() + a.1.value())
        .sum()
}


fn get_input_data(path: &str) -> String {
    fs::read_to_string(format!("data/day02/{path}", path = path)).expect("File not found")
}

fn convert_to_move(encoded_move: &str) -> Move {
    match encoded_move.to_uppercase().as_str() {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => panic!("Invalid move"),
    }
}

fn convert_to_result(encoded_result: &str) -> Result {
    match encoded_result.to_uppercase().as_str() {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("Invalid move"),
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self, other: &Move) -> Result {
        match (self, other) {
            (Rock, Scissors) => Win,
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
            (Scissors, Rock) => Lose,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
        }
    }

    fn result_obtained_by(&self, result: &Result) -> Move {
        match (self, result) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Lose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Lose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Lose) => Paper,
        }
    }
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn value(&self) -> u32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}
