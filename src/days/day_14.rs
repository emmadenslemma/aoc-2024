use std::{collections::HashMap, convert::identity};

use itertools::Itertools;
use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn get_input() -> Vec<((i32, i32), (i32, i32))> {
    let input = std::fs::read_to_string("inputs/day_14.txt").unwrap();
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    re.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [px, py, vx, vy])| {
            (
                (px.parse().unwrap(), py.parse().unwrap()),
                (vx.parse().unwrap(), vy.parse().unwrap()),
            )
        })
        .collect()
}

fn move_robot(px: i32, py: i32, vx: i32, vy: i32, distance: i32) -> (i32, i32) {
    (
        ((px + vx * distance) % WIDTH + WIDTH) % WIDTH,
        ((py + vy * distance) % HEIGHT + HEIGHT) % HEIGHT,
    )
}

fn get_quadrant(px: i32, py: i32) -> Option<i32> {
    if px == WIDTH / 2 || py == HEIGHT / 2 {
        None
    } else if px > WIDTH / 2 {
        if py > HEIGHT / 2 {
            Some(1)
        } else {
            Some(2)
        }
    } else {
        if py > HEIGHT / 2 {
            Some(3)
        } else {
            Some(4)
        }
    }
}

pub fn part1() -> i64 {
    let robots = get_input();

    let mut quadrant_count: HashMap<i32, i32> = HashMap::new();

    robots
        .into_iter()
        .map(|((px, py), (vx, vy))| move_robot(px, py, vx, vy, 100))
        .map(|(px, py)| get_quadrant(px, py))
        .filter_map(identity)
        .for_each(|quadrant| *quadrant_count.entry(quadrant).or_default() += 1);

    let ones = *quadrant_count.get(&1).unwrap_or(&0);
    let twos = *quadrant_count.get(&2).unwrap_or(&0);
    let threes = *quadrant_count.get(&3).unwrap_or(&0);
    let fours = *quadrant_count.get(&4).unwrap_or(&0);

    (ones * twos * threes * fours) as i64
}

pub fn part2() -> i64 {
    let robots = get_input();

    // it doesn't have to be pretty it just has to be over
    for i in 0..10000 {
        let mut display = vec![vec![0; HEIGHT as usize]; WIDTH as usize];

        for (px, py) in robots
            .iter()
            .map(|((px, py), (vx, vy))| move_robot(*px, *py, *vx, *vy, i))
        {
            display[px as usize][py as usize] += 1;
        }

        // Christmas tree should have horizontal contiguous lines through it
        if !display
            .iter()
            .flatten()
            .tuple_windows()
            .any(|(a, b, c, d, e, f, g, h, i, j)| {
                a != &0
                    && b != &0
                    && c != &0
                    && d != &0
                    && e != &0
                    && f != &0
                    && g != &0
                    && h != &0
                    && i != &0
                    && j != &0
            })
        {
            continue;
        }

        let mut has_tip = false;

        for x in 0..WIDTH as usize - 4 {
            for y in 0..HEIGHT as usize - 3 {
                if display[x + 2][y] == 0
                    && display[x + 1][y + 1] == 0
                    && display[x + 2][y + 1] == 1
                    && display[x + 3][y + 1] == 0
                    && display[x][y + 2] == 0
                    && display[x + 1][y + 2] == 1
                    && display[x + 3][y + 2] == 1
                    && display[x + 4][y + 2] == 0
                    && display[x][y + 3] == 1
                    && display[x + 4][y + 3] == 1
                {
                    has_tip = true;
                }
            }
        }

        if !has_tip {
            continue;
        }

        for y in 0..HEIGHT {
            println!();
            for x in 0..WIDTH {
                let num = display[x as usize][y as usize];
                print!(
                    "{}",
                    if num == 0 {
                        ' '
                    } else {
                        char::from_digit(num, 10).unwrap()
                    }
                );
            }
        }

        return i as i64;
    }

    0
}
