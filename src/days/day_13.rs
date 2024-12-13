use std::convert::identity;

use regex::Regex;

fn get_input() -> Vec<(f64, f64, f64, f64, f64, f64)> {
    let input = std::fs::read_to_string("inputs/day_13.txt").unwrap();
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    re.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, d, b, e, c, f])| {
            (
                a.parse().unwrap(),
                b.parse().unwrap(),
                c.parse().unwrap(),
                d.parse().unwrap(),
                e.parse().unwrap(),
                f.parse().unwrap(),
            )
        })
        .collect()
}

fn solve((a, b, c, d, e, f): (f64, f64, f64, f64, f64, f64)) -> Option<f64> {
    let y = ((f - d * c / a) / (e - d * b / a)).round();
    let x = ((c - b * y) / a).round();
    if a * x + b * y == c && d * x + e * y == f {
        Some(x * 3.0 + y)
    } else {
        None
    }
}

pub fn part1() -> i64 {
    let equations = get_input();

    equations
        .into_iter()
        .map(|args| solve(args))
        .filter_map(identity)
        .sum::<f64>() as i64
}

pub fn part2() -> i64 {
    let equations = get_input();

    let offset = 10000000000000.0;

    equations
        .into_iter()
        .map(|(a, b, c, d, e, f)| (a, b, c + offset, d, e, f + offset))
        .map(|args| solve(args))
        .filter_map(identity)
        .sum::<f64>() as i64
}
