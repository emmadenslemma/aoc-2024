#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

impl Direction {
    pub fn rotate_90_deg(&self) -> Self {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    pub fn forward(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        Some(match self {
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x.checked_sub(1)?, y),
            Up => (x, y.checked_sub(1)?),
        })
    }
}
