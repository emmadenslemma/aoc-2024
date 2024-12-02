use std::{error::Error, fs::read_to_string};

fn parse_input() -> Vec<Vec<i32>> {
    let input = read_to_string("inputs/day_02.txt").unwrap();

    input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let input = parse_input();

    let result = input.iter().filter(|level| is_level_safe(level)).count();

    Ok(result as i32)
}

fn is_level_safe(level: &Vec<i32>) -> bool {
    level.windows(3).all(|slice| {
        let a = slice[0];
        let b = slice[1];
        let c = slice[2];

        let ab_diff = (a - b).abs();
        let bc_diff = (b - c).abs();

        ((a > b && b > c) || (a < b && b < c))
            && ab_diff > 0
            && ab_diff < 4
            && bc_diff > 0
            && bc_diff < 4
    })
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let input = parse_input();

    let result = input
        .iter()
        .filter(|level| {
            (0..level.len()).any(|i| {
                let mut damp_level = (*level).clone();

                damp_level.remove(i);

                is_level_safe(&damp_level)
            })
        })
        .count();

    Ok(result as i32)
}
