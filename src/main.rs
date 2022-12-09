use solutions::day01::part1 as day01_part1;
use solutions::day01::part2 as day01_part2;
use solutions::day02::part1 as day02_part1;
use solutions::day02::part2 as day02_part2;
use solutions::day03::part1 as day03_part1;
use solutions::day03::part2 as day03_part2;
use solutions::day04::part1 as day04_part1;
use solutions::day04::part2 as day04_part2;

mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}

fn main() {
    println!("Day 1, part 1: {}", day01_part1());
    println!("Day 1, part 2: {}", day01_part2());
    println!("Day 2, part 1: {}", day02_part1());
    println!("Day 2, part 2: {}", day02_part2());
    println!("Day 3, part 1: {}", day03_part1());
    println!("Day 3, part 2: {}", day03_part2());
    println!("Day 4, part 1: {}", day04_part1());
    println!("Day 4, part 2: {}", day04_part2());
}
