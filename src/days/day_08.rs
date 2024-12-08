use std::collections::HashMap;
// Only took 8 days for me to add itertools
use itertools::Itertools;

struct Map {
    // Wow I really thought I was gonna need that
    #[allow(dead_code)]
    // X and Y are inverted for performance/simplicity's sake.
    // index via `self.antennas[y][x]`
    antennas: Vec<Vec<char>>,
    frequencies: HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
}

impl Map {
    pub fn new_from_string(s: &str) -> Self {
        let mut frequencies = HashMap::new();
        let antennas: Vec<Vec<char>> = s
            .lines()
            .enumerate()
            .map(|(y, xs)| {
                xs.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c != '.' {
                            if !frequencies.contains_key(&c) {
                                frequencies.insert(c, Vec::new());
                            }
                            frequencies.get_mut(&c).unwrap().push((y as i32, x as i32));
                        }
                        c
                    })
                    .collect()
            })
            .collect();

        Map {
            frequencies,
            height: antennas.len() as i32,
            width: antennas[0].len() as i32,
            antennas,
        }
    }

    pub fn is_in_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && x >= 0 && x < self.width && y < self.height
    }
}

fn get_input() -> Map {
    let input = std::fs::read_to_string("inputs/day_08.txt").unwrap();

    Map::new_from_string(&input)
}

pub fn part1() -> i64 {
    let map = get_input();

    map.frequencies
        .iter()
        .map(|(_, coords)| {
            coords
                .iter()
                .permutations(2)
                .map(|p| {
                    let ((y, x), (y2, x2)) = p.iter().next_tuple().unwrap();

                    (y - (y2 - y), x - (x2 - x))
                })
                .filter(|(y, x)| map.is_in_bounds(*y, *x))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .unique()
        .count() as i64
}

pub fn part2() -> i64 {
    let map = get_input();

    map.frequencies
        .iter()
        .map(|(_, coords)| {
            coords
                .iter()
                .permutations(2)
                .map(|p| {
                    let ((y, x), (y2, x2)) = p.iter().next_tuple().unwrap();

                    let mut antinodes = Vec::new();
                    let (vy, vx) = (y2 - y, x2 - x);
                    let (mut ay, mut ax) = (*y, *x);

                    while map.is_in_bounds(ay, ax) {
                        antinodes.push((ay, ax));

                        ay -= vy;
                        ax -= vx;
                    }

                    antinodes
                })
                .flatten()
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .unique()
        .count() as i64
}
