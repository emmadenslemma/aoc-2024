mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

pub fn run_solution(day: i32) -> (i64, i64) {
    match day {
        1 => (day_01::part1() as i64, day_01::part2() as i64),
        2 => (day_02::part1() as i64, day_02::part2() as i64),
        3 => (day_03::part1() as i64, day_03::part2() as i64),
        4 => (day_04::part1() as i64, day_04::part2() as i64),
        5 => (day_05::part1() as i64, day_05::part2() as i64),
        6 => (day_06::part1() as i64, day_06::part2() as i64),
        7 => (day_07::part1(), day_07::part2()),
        8 => (day_08::part1(), day_08::part2()),
        9 => (day_09::part1(), day_09::part2()),
        10 => (day_10::part1(), day_10::part2()),
        11 => (day_11::part1(), day_11::part2()),
        12 => (day_12::part1(), day_12::part2()),
        13 => (day_13::part1(), day_13::part2()),
        14 => (day_14::part1(), day_14::part2()),
        15 => (day_15::part1(), day_15::part2()),
        _ => panic!(),
    }
}
