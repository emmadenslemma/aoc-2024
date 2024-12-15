#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn from_arrow(a: char) -> Self {
        use Direction::*;
        match a {
            '>' => Right,
            'v' => Down,
            '<' => Left,
            '^' => Up,
            c => panic!("Failed to parse unrecognized arrow '{c}'"),
        }
    }
}

// Day 15 never moves out of bounds so we don't have to check it
pub fn move_in_direction(dir: Direction, x: usize, y: usize) -> (usize, usize) {
    use Direction::*;
    match dir {
        Right => (x + 1, y),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Up => (x, y - 1),
    }
}
