mod map;

use map::*;

fn get_input() -> Map {
    let input = std::fs::read_to_string("inputs/day_10.txt").unwrap();

    Map::from(input.as_str())
}

pub fn part1() -> i64 {
    let map = get_input();

    map.heights
        .iter()
        .enumerate()
        .map(|(x, ys)| {
            ys.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(|(y, _)| map.get_score_from_trailhead(x, y))
                .sum::<usize>()
        })
        .sum::<usize>() as i64
}

pub fn part2() -> i64 {
    let map = get_input();
    // This is the exact same iterator soup from part 1, and I should be extracting it into a generic higher order function
    // but really who has the time
    map.heights
        .iter()
        .enumerate()
        .map(|(x, ys)| {
            ys.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(|(y, _)| map.get_rating_from_trailhead(x, y))
                .sum::<usize>()
        })
        .sum::<usize>() as i64
}
