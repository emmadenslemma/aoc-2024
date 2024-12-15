#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    Empty,
    Obstacle,
    Box,
    BigBoxLeft,
    BigBoxRight,
}

impl From<char> for Entity {
    fn from(c: char) -> Self {
        use Entity::*;
        match c {
            '#' => Obstacle,
            'O' => Box,
            '[' => BigBoxLeft,
            ']' => BigBoxRight,
            _ => Empty,
        }
    }
}

impl Into<char> for Entity {
    fn into(self) -> char {
        use Entity::*;
        match self {
            Obstacle => '#',
            Box => 'O',
            BigBoxLeft => '[',
            BigBoxRight => ']',
            Empty => '.',
        }
    }
}

impl Entity {
    pub fn is_empty(self) -> bool {
        self == Entity::Empty
    }

    pub fn is_box(self) -> bool {
        use Entity::*;
        self == Box || self == BigBoxLeft || self == BigBoxRight
    }
}
