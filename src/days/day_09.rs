// When asked about his extensive iterator overuse,
// Alexander the Great famously replied
//   "What the fuck is an iterator?"

// Okay real talk this might be the worst code I've ever written

fn get_input() -> Vec<Option<usize>> {
    let input = std::fs::read_to_string("inputs/day_09.txt").unwrap();

    input
        .trim()
        .chars()
        .zip(input.chars().skip(1))
        .enumerate()
        .filter(|(n, _)| n % 2 == 0)
        .enumerate()
        .map(|(id, (_, (file, free_space)))| (id, file, free_space))
        .map(|(id, file, free_space)| {
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

fn get_inputs_in_blocks() -> (Vec<(usize, usize, usize)>, Vec<(usize, usize, usize)>) {
    let input = std::fs::read_to_string("inputs/day_09.txt").unwrap();

    // Forgive me father for I have state
    let mut pos = 0;

    input
        .trim()
        .chars()
        .zip(input.chars().skip(1))
        .enumerate()
        .filter(|(n, _)| n % 2 == 0)
        .enumerate()
        .map(|(id, (_, (file, free_space)))| (id, file, free_space))
        .map(|(id, file, free_space)| {
            let parse = |c: char| c.to_digit(10).or(Some(0)).unwrap() as usize;
            let file_metadata = (id, parse(file), pos);
            pos += file_metadata.1;
            let space_metadata = (id, parse(free_space), pos);
            pos += space_metadata.1;
            (file_metadata, space_metadata)
        })
        .unzip()
}

fn leftpad_memory_but_better(
    file_blocks: &mut Vec<(usize, usize, usize)>,
    free_space_blocks: &mut Vec<(usize, usize, usize)>,
) {
    file_blocks.iter_mut().rev().for_each(|file| {
        if let Some(storage) = free_space_blocks
            .iter_mut()
            .find(|(_, storage_size, storage_pos)| file.2 > *storage_pos && *storage_size >= file.1)
        {
            file.2 = storage.2;
            storage.1 -= file.1;
            storage.2 += file.1;
        }
    });
}

fn calculate_block_checksum(file_blocks: &Vec<(usize, usize, usize)>) -> i64 {
    file_blocks
        .iter()
        .map(|(id, size, pos)| {
            (*pos..(pos + size))
                .map(|pos| pos as i64 * *id as i64)
                .sum::<i64>()
        })
        .sum()
}

pub fn part1() -> i64 {
    let mut disk = get_input();

    leftpad_memory(&mut disk);

    calculate_checksum(&disk)
}

pub fn part2() -> i64 {
    let (mut file_blocks, mut free_space_blocks) = get_inputs_in_blocks();

    leftpad_memory_but_better(&mut file_blocks, &mut free_space_blocks);

    calculate_block_checksum(&file_blocks)
}
