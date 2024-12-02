use std::iter::zip;

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string("inputs/day_01.txt").unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    input
        .lines()
        .map(|l| l.split("   ").collect::<Vec<&str>>())
        .for_each(|v| {
            let a = v[0].parse().unwrap();
            let b = v[1].parse().unwrap();

            left.push(a);
            right.push(b);
        });

    (left, right)
}

pub fn part1() -> i32 {
    let (mut left, mut right) = parse_input();

    left.sort();
    right.sort();

    zip(left, right).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part2() -> i32 {
    let (left, right) = parse_input();

    left.iter()
        .map(|x| right.iter().fold(0, |n, y| if x == y { n + 1 } else { n }) * x)
        .sum()
}
