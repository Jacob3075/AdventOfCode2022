use std::fs;

pub fn part1() -> u32 {
    fs::read_to_string("data/day04/input.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|parts| Pair {
                    first: parse_assignment_range(parts.0),
                    second: parse_assignment_range(parts.1),
                })
                .unwrap()
        })
        .map(|pair| pair.is_any_fully_contained())
        .filter(|&x| x)
        .count() as u32
}

pub fn part2() -> u32 {
    0
}

fn parse_assignment_range(range: &str) -> AssignmentRange {
    let pair = range.split_once("-")
        .map(|nums| {
            (nums.0.parse::<u32>().unwrap(), nums.1.parse::<u32>().unwrap())
        })
        .unwrap();

    AssignmentRange {
        start: pair.0,
        end: pair.1,
    }
}

struct Pair {
    first: AssignmentRange,
    second: AssignmentRange,
}

struct AssignmentRange {
    start: u32,
    end: u32,
}

impl Pair {
    fn is_any_fully_contained(&self) -> bool {
        self.first.is_fully_contained(&self.second) || self.second.is_fully_contained(&self.first)
    }
}

impl AssignmentRange {
    fn is_fully_contained(&self, other: &AssignmentRange) -> bool {
        let range = other.start ..=other.end;
        range.contains(&self.start) && range.contains(&self.end)
    }
}
