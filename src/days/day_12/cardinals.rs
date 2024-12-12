use std::convert::identity;

pub fn cardinals(x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
    [
        x.checked_add(1).map(|x| (x, y)),
        y.checked_add(1).map(|y| (x, y)),
        x.checked_sub(1).map(|x| (x, y)),
        y.checked_sub(1).map(|y| (x, y)),
    ]
}

pub fn cardinals_iter(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    cardinals(x, y).into_iter().filter_map(identity)
}
