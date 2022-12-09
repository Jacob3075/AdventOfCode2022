use std::fs;

pub fn part1() -> u32 {
    let result = get_input_data()
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(sum_elements_in_group)
        .max()
        .unwrap();
    result
}

pub fn part2() -> u32 {
    let mut sum_of_groups = get_input_data()
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(sum_elements_in_group)
        .collect::<Vec<u32>>();

    sum_of_groups.sort();
    sum_of_groups.reverse();

    sum_of_groups.iter().take(3).sum()
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
