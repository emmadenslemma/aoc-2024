use std::collections::HashMap;

pub fn part1() -> i64 {
    let map = get_input();

    let new_map = iterate_n_times(map, 25);

    new_map.into_values().sum::<u64>() as i64
}

pub fn part2() -> i64 {
    let map = get_input();

    let new_map = iterate_n_times(map, 75);

    new_map.into_values().sum::<u64>() as i64
}

fn get_input() -> HashMap<i64, u64> {
    let input = std::fs::read_to_string("inputs/day_11.txt").unwrap();

    // Yes, this assumes I have no duplicates in my puzzle input
    input
        .trim()
        .split(' ')
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn iterate_n_times(mut numbers: HashMap<i64, u64>, n: usize) -> HashMap<i64, u64> {
    for _ in 0..n {
        numbers = iterate(numbers);
    }

    numbers
}

fn iterate(numbers: HashMap<i64, u64>) -> HashMap<i64, u64> {
    let mut next_numbers = HashMap::new();

    let mut add = |n, x| next_numbers.insert(n, next_numbers.get(&n).map(|v| v + x).unwrap_or(x));

    numbers.iter().for_each(|(k, v)| {
        match k {
            0 => {
                add(1, *v);
            }
            n if digits(*n) % 2 == 0 => {
                let (a, b) = split_number(*n);

                add(a, *v);
                add(b, *v);
            }
            n => {
                add(n * 2024, *v);
            }
        };
    });

    next_numbers
}

fn digits(n: i64) -> u32 {
    1 + if n > 0 { n.ilog10() } else { 0 }
}

fn split_number(n: i64) -> (i64, i64) {
    let split_at = 10i64.pow(digits(n) / 2);
    let left = n / split_at;
    let right = n - split_at * left;

    (left, right)
}
