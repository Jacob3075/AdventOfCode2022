use std::collections::HashSet;
use std::fs;

pub fn part1() -> u32 {
    fs::read_to_string("data/day03/input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            let first_half = first_half.chars().into_iter().collect::<HashSet<_>>();
            let second_half = second_half.chars().into_iter().collect::<HashSet<_>>();
            (first_half, second_half)
        })
        .flat_map(|(first_half, second_half)| {
            first_half.intersection(&second_half)
                .map(get_value_for_char)
                .nth(0)
        })
        .sum()
}

pub fn part2() -> u32 {
    fs::read_to_string("data/day03/input.txt")
        .expect("Unable to read file")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            lines.into_iter()
                .map(|line| (*line).to_owned())
                .map(|line| line.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
        })
        .flat_map(|elf_group| {
            elf_group.into_iter()
                .fold(HashSet::new(), |acc, line| {
                    if acc.len() == 0 { return line; };

                    acc.intersection(&line)
                        .map(|c| c.to_owned())
                        .collect()
                })
                .into_iter()
                .nth(0)
        })
        .map(|a| get_value_for_char(&a))
        .sum()
}

fn get_value_for_char(letter: &char) -> u32 {
    match letter.is_uppercase() {
        true => *letter as u32 - 'A' as u32 + 27,
        false => *letter as u32 - 'a' as u32 + 1,
    }
}
