// When asked about his extensive iterator overuse,
// Alexander the Great famously replied
//   "What the fuck is an iterator?"
//   "We're invading Egypt, go grab a sword instead of asking dumb questions"

fn get_input() -> Vec<Option<usize>> {
    let input = std::fs::read_to_string("inputs/day_09.txt").unwrap();

    input
        .trim()
        .chars()
        .zip(input.chars().skip(1))
        .enumerate()
        .filter(|(n, _)| n % 2 == 0)
        .enumerate()
        .map(|(id, (_, (file, free_space)))| (id, (file, free_space)))
        .map(|(id, (file, free_space))| {
            let to_usize = |c: char| c.to_digit(10).map(|n| n as usize);
            let mut disk = Vec::new();

            if let Some(file_len) = to_usize(file) {
                disk.append(&mut vec![Some(id); file_len]);

                if let Some(free_space_len) = to_usize(free_space) {
                    disk.append(&mut vec![None; free_space_len]);
                }
            }

            disk
        })
        .flatten()
        .collect()
}

fn leftpad_memory(disk: &mut Vec<Option<usize>>) {
    let empty_space_indexes: Vec<usize> = disk
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_none())
        .map(|(i, _)| i)
        .collect();
    let filled_space_indexes: Vec<usize> = disk
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, value)| value.is_some())
        .map(|(i, _)| i)
        .collect();

    empty_space_indexes
        .iter()
        .zip(filled_space_indexes.iter())
        .filter(|(i, j)| i < j)
        .for_each(|(i, j)| disk.swap(*i, *j));
}

fn calculate_checksum(disk: &Vec<Option<usize>>) -> i64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, data)| {
            if let Some(file) = data {
                Some((i, file))
            } else {
                None
            }
        })
        .map(|(i, file)| i as i64 * *file as i64)
        .sum()
}

pub fn part1() -> i64 {
    let mut disk = get_input();

    leftpad_memory(&mut disk);

    calculate_checksum(&disk)
}

pub fn part2() -> i64 {
    0
}
