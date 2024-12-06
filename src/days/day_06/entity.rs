#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    Empty,
    Obstacle,
    Guard,
    Visited,
}

impl From<char> for Entity {
    fn from(c: char) -> Self {
        use Entity::*;
        match c {
            '^' => Guard,
            '#' => Obstacle,
            _ => Empty,
        }
    }
}
