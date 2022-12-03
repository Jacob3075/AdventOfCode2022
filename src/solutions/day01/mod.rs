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

pub fn part2() {
    let mut sum_of_groups = get_input_data()
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(sum_elements_in_group)
        .collect::<Vec<u32>>();

    sum_of_groups.sort();
    sum_of_groups.reverse();

    let result: u32 = sum_of_groups.iter().take(3).sum();

    println!("{result}")
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
