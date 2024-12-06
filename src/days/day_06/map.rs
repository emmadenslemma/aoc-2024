use crate::days::day_06::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Left,
    Looped,
}

#[derive(Debug, Default, Clone)]
pub struct Map {
    guard_pos: (usize, usize),
    direction: Direction,
    pub entities: Vec<Vec<Entity>>,
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
    pub fn width(&self) -> usize {
        self.entities.len()
    }

    pub fn height(&self) -> usize {
        self.entities[0].len()
    }

    pub fn move_to_end(&mut self) -> MoveResult {
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
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.entities[x][y] == entity {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn pos_in_front(&self) -> Option<(usize, usize)> {
        let (x, y) = self.direction.forward(self.guard_pos)?;
        if x < self.width() && y < self.height() {
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

    pub fn get_visited_tile_count(&self) -> usize {
        self.entities
            .iter()
            .flatten()
            .filter(|e| **e == Entity::Visited)
            .count()
    }
}
