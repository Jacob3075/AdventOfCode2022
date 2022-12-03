use std::fs;

pub fn part1() {
    let result = get_input_data()
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(sum_elements_in_group)
        .max();

    match result {
        Some(expr) => println!("{expr}"),
        None => println!("No value found"),
    };
}

fn get_input_data() -> String {
    fs::read_to_string("data/day01/input.txt").expect("File not found")
}

fn sum_elements_in_group(group: &str) -> u32 {
    group
        .split("\n")
        .map(|line| line.parse::<u32>().expect("not a valid number"))
        .sum()
}
