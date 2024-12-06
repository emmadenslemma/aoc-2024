mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

pub fn run_solution(day: i32) -> (i32, i32) {
    match day {
        1 => (day_01::part1(), day_01::part2()),
        2 => (day_02::part1(), day_02::part2()),
        3 => (day_03::part1(), day_03::part2()),
        4 => (day_04::part1(), day_04::part2()),
        5 => (day_05::part1(), day_05::part2()),
        6 => (day_06::part1(), day_06::part2()),
        _ => panic!(),
    }
}
