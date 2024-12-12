mod cardinals;
mod garden;

use garden::*;

fn get_input() -> Garden {
    let input = std::fs::read_to_string("inputs/day_12.txt").unwrap();

    Garden::new(&input)
}

pub fn part1() -> i64 {
    let garden = get_input();

    garden.calculate_price() as i64
}

pub fn part2() -> i64 {
    let garden = get_input();

    garden.calculate_bulk_price() as i64
}
