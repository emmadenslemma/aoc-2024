mod direction;
mod entity;
mod warehouse;

use warehouse::*;

fn get_input() -> Warehouse {
    let input = std::fs::read_to_string("inputs/day_15.txt").unwrap();

    Warehouse::new(&input)
}

pub fn part1() -> i64 {
    let mut warehouse = get_input();

    warehouse.apply_movements();

    warehouse.sum_box_coordinates() as i64
}

pub fn part2() -> i64 {
    let mut warehouse = get_input();

    warehouse.embiggen();

    warehouse.apply_movements();

    warehouse.sum_box_coordinates() as i64
}
