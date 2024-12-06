#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    Empty,
    Obstacle,
    Guard,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveResult {
    Left,
    Looped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
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
    fn rotate_90_deg(&self) -> Self {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn forward(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        // Yes, this is taking the inverted values into account
        Some(match self {
            Right => (x, y + 1),
            Down => (x + 1, y),
            Left => (x, y.checked_sub(1)?),
            Up => (x.checked_sub(1)?, y),
        })
    }
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

#[derive(Debug, Default, Clone)]
struct Map {
    guard_pos: (usize, usize),
    direction: Direction,
    entities: Vec<Vec<Entity>>,
    turns: Vec<((usize, usize), Direction)>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut map = Map {
            entities: s
                .lines()
                // Oopsie! This line inverts X and Y!
                .map(|l| l.chars().map(|c| Entity::from(c)).collect())
                .collect(),
            ..Default::default()
        };

        if let Some((x, y)) = map.find_entity(Entity::Guard) {
            map.move_guard(x, y);
        }

        map
    }
}

impl Map {
    fn map_width(&self) -> usize {
        self.entities.len()
    }

    fn map_height(&self) -> usize {
        self.entities[0].len()
    }

    fn move_to_end(&mut self) -> MoveResult {
        if let Some((x, y)) = self.pos_in_front() {
            if self.is_blocked(x, y) {
                if self.turns.contains(&(self.guard_pos, self.direction)) {
                    return MoveResult::Looped;
                }
                self.turn();
            } else {
                self.move_guard(x, y);
            }

            self.move_to_end()
        } else {
            MoveResult::Left
        }
    }

    fn find_entity(&self, entity: Entity) -> Option<(usize, usize)> {
        for x in 0..self.map_width() {
            for y in 0..self.map_height() {
                if self.entities[x][y] == entity {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn pos_in_front(&self) -> Option<(usize, usize)> {
        let (x, y) = self.direction.forward(self.guard_pos)?;
        if x < self.map_width() && y < self.map_height() {
            Some((x, y))
        } else {
            None
        }
    }

    fn move_guard(&mut self, x: usize, y: usize) {
        self.guard_pos = (x, y);
        self.entities[x][y] = Entity::Visited;
    }

    fn turn(&mut self) {
        self.turns.push((self.guard_pos, self.direction));
        self.direction = self.direction.rotate_90_deg();
    }

    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.entities[x][y] == Entity::Obstacle
    }

    fn get_visited_tile_count(&self) -> usize {
        self.entities
            .iter()
            .flatten()
            .filter(|e| **e == Entity::Visited)
            .count()
    }
}

fn get_input() -> Map {
    let input = std::fs::read_to_string("inputs/day_06.txt").unwrap();

    Map::from(input.as_str())
}

pub fn part1() -> i32 {
    let mut map = get_input();

    map.move_to_end();

    map.get_visited_tile_count() as i32
}

pub fn part2() -> i32 {
    let map = get_input();

    let mut loop_count = 0;

    for x in 0..map.map_width() {
        for y in 0..map.map_height() {
            let mut cur_map = map.clone();
            if cur_map.entities[x][y] == Entity::Empty {
                cur_map.entities[x][y] = Entity::Obstacle;
            } else {
                continue;
            }

            if cur_map.move_to_end() == MoveResult::Looped {
                loop_count += 1;
            }
        }
    }

    loop_count
}