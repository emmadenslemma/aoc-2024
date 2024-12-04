fn get_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("inputs/day_04.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

pub fn part1() -> i32 {
    let input = get_input();

    let mut total = 0;

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            total += check_word(&input, x, y);
        }
    }

    total
}

pub fn part2() -> i32 {
    let input = get_input();

    let mut total = 0;

    for x in 1..input.len() - 1 {
        for y in 1..input[x].len() - 1 {
            if is_word_x_mas(&input, x, y) {
                total += 1;
            }
        }
    }

    total
}

fn check_word(v: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut matches = 0;

    for xmod in -1..=1 {
        for ymod in -1..=1 {
            if check_direction(String::new(), v, x, y, xmod, ymod) {
                matches += 1;
            }
        }
    }

    matches
}

fn check_direction(
    mut w: String,
    v: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    xmod: i32,
    ymod: i32,
) -> bool {
    w.push(v[x][y]);

    if w == "XMAS" {
        return true;
    }

    if can_word_be_xmas(&w)
        && !(x == 0 && xmod == -1)
        && !(x == v.len() - 1 && xmod == 1)
        && !(y == 0 && ymod == -1)
        && !(y == v[x].len() - 1 && ymod == 1)
    {
        check_direction(
            w,
            v,
            (x as i32 + xmod) as usize,
            (y as i32 + ymod) as usize,
            xmod,
            ymod,
        )
    } else {
        false
    }
}

fn can_word_be_xmas(w: &String) -> bool {
    w.chars().zip("XMAS".chars()).all(|(a, b)| a == b)
}

fn is_word_x_mas(v: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    v[x][y] == 'A'
        && ((v[x - 1][y - 1] == 'M' && v[x + 1][y + 1] == 'S')
            || (v[x - 1][y - 1] == 'S' && v[x + 1][y + 1] == 'M'))
        && ((v[x + 1][y - 1] == 'M' && v[x - 1][y + 1] == 'S')
            || (v[x + 1][y - 1] == 'S' && v[x - 1][y + 1] == 'M'))
}
