use rayon::prelude::*;

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
        Some(match self {
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x.checked_sub(1)?, y),
            Up => (x, y.checked_sub(1)?),
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
        let width = s.find('\n').unwrap();
        let mut entities = vec![vec![]; width];

        s.replace('\n', "")
            .chars()
            .map(|c| Entity::from(c))
            .enumerate()
            .for_each(|(x, e)| entities[x % width].push(e));

        let mut map = Map {
            entities,
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

    (0..map.map_width())
        .into_par_iter()
        .map(|x| {
            (0..map.map_height())
                .into_par_iter()
                .filter(|y| {
                    let mut cur_map = map.clone();

                    if cur_map.entities[x][*y] == Entity::Empty {
                        cur_map.entities[x][*y] = Entity::Obstacle;

                        return cur_map.move_to_end() == MoveResult::Looped;
                    }

                    false
                })
                .count()
        })
        .sum::<usize>() as i32
}
