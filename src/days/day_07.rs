use rayon::prelude::*;

fn get_input() -> Vec<(i64, Vec<i32>)> {
    let input = std::fs::read_to_string("inputs/day_07.txt").unwrap();

    input
        .par_lines()
        .map(|l| l.split(": "))
        .map(|mut spliterator| {
            let result = spliterator.next().unwrap().parse().unwrap();
            let numbers = spliterator
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            (result, numbers)
        })
        .collect()
}

fn concat_numbers(a: i64, b: i64) -> i64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn calc_recursively(
    expected_result: i64,
    accumulator: i64,
    numbers: &[i32],
    use_third_operator: bool,
) -> bool {
    let cur_number = numbers[0] as i64;
    if numbers.len() == 1 {
        accumulator + cur_number == expected_result
            || accumulator * cur_number == expected_result
            || (use_third_operator && concat_numbers(accumulator, cur_number) == expected_result)
    } else {
        calc_recursively(
            expected_result,
            accumulator + cur_number,
            &numbers[1..],
            use_third_operator,
        ) || calc_recursively(
            expected_result,
            accumulator * cur_number,
            &numbers[1..],
            use_third_operator,
        ) || (use_third_operator
            && calc_recursively(
                expected_result,
                concat_numbers(accumulator, cur_number),
                &numbers[1..],
                use_third_operator,
            ))
    }
}

pub fn part1() -> i64 {
    let input = get_input();

    input
        .par_iter()
        .filter(|(result, numbers)| {
            calc_recursively(*result, numbers[0] as i64, &numbers[1..], false)
        })
        .map(|(result, _)| result)
        .sum()
}

pub fn part2() -> i64 {
    let input = get_input();

    input
        .par_iter()
        .filter(|(result, numbers)| {
            calc_recursively(*result, numbers[0] as i64, &numbers[1..], true)
        })
        .map(|(result, _)| result)
        .sum()
}
