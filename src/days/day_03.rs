use regex::Regex;

fn get_input() -> String {
    std::fs::read_to_string("inputs/day_03.txt").unwrap()
}

pub fn part1() -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = get_input();

    let to_int = |s: &str| s.parse::<i32>().unwrap();

    regex
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| to_int(a) * to_int(b))
        .sum()
}

pub fn part2() -> i32 {
    let regex = Regex::new(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))").unwrap();
    let input = get_input();

    let captured_int =
        |c: &regex::Captures, i: usize| c.get(i).unwrap().as_str().parse::<i32>().unwrap();

    let mut total = 0;
    let mut disable_next_mul = false;

    for c in regex.captures_iter(&input) {
        match c.get(1).unwrap().as_str() {
            "don't()" => disable_next_mul = true,
            "do()" => disable_next_mul = false,
            _ => {
                if !disable_next_mul {
                    let a = captured_int(&c, 2);
                    let b = captured_int(&c, 3);

                    total += a * b;
                }
            }
        }
    }

    total
}
