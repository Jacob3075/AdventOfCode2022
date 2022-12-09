use solutions::day01::part1 as day01_part1;
use solutions::day01::part2 as day01_part2;
use solutions::day02::part1 as day02_part1;
use solutions::day02::part2 as day02_part2;

mod solutions {
    pub mod day01;
    pub mod day02;
}

fn main() {
    println!("Day 2, part 1: {}", day02_part1());
    println!("Day 2, part 2: {}", day02_part2());
}
