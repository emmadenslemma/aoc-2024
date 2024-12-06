fn get_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input = std::fs::read_to_string("inputs/day_05.txt").unwrap();

    let divider_pos = input.find("\n\n").unwrap();

    let ordering_rules = (&input[0..divider_pos])
        .lines()
        .map(|s| {
            let mut spliterator = s.split('|');

            let mut next_int = || spliterator.next().unwrap().parse().unwrap();

            let left = next_int();
            let right = next_int();

            (left, right)
        })
        .collect();

    let page_updates = (&input[divider_pos..input.len()])
        .trim()
        .lines()
        .map(|s| s.split(",").map(|d| d.parse().unwrap()).collect())
        .collect();

    (ordering_rules, page_updates)
}

pub fn part1() -> i32 {
    let (ordering_rules, page_updates) = get_input();

    page_updates
        .iter()
        .filter(|page_update| {
            page_update.windows(2).all(|slice| {
                let cur_page = &slice[0];
                let next_page = &slice[1];
                !ordering_rules
                    .iter()
                    .any(|(left, right)| left == next_page && right == cur_page)
            })
        })
        .map(|page_update| page_update[page_update.len() / 2])
        .sum()
}

pub fn part2() -> i32 {
    let (ordering_rules, mut page_updates) = get_input();

    let mut total = 0;

    for page_update in page_updates.iter_mut() {
        let mut modified = false;

        for i in 0..page_update.len() - 1 {
            for j in i + 1..page_update.len() {
                let cur_page = page_update[i];
                let next_page = page_update[j];

                if ordering_rules
                    .iter()
                    .any(|(left, right)| *left == next_page && *right == cur_page)
                {
                    page_update.swap(i, j);

                    modified = true;
                }
            }
        }

        if modified {
            total += page_update[page_update.len() / 2];
        }
    }

    total
}
