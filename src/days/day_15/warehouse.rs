use super::direction::{move_in_direction, Direction};
use super::entity::Entity;

pub struct Warehouse {
    arena: Vec<Vec<Entity>>,
    robot_pos: (usize, usize),
    movements: Vec<Direction>,
}

impl Warehouse {
    pub fn new(s: &str) -> Self {
        let (arena_str, movements_str) = split_string(s, "\n\n");

        let width = arena_str.lines().next().unwrap().len();
        let height = arena_str.lines().count();

        let mut arena = vec![vec![Entity::Empty; height]; width];
        let mut robot_pos = (0, 0);

        arena_str.lines().enumerate().for_each(|(y, xs)| {
            xs.chars().enumerate().for_each(|(x, c)| {
                arena[x][y] = Entity::from(c);
                if c == '@' {
                    robot_pos = (x, y);
                }
            });
        });

        let movements = movements_str
            .lines()
            .map(|l| l.chars())
            .flatten()
            .map(|c| Direction::from_arrow(c))
            .collect();

        Warehouse {
            arena,
            robot_pos,
            movements,
        }
    }

    fn width(&self) -> usize {
        self.arena.len()
    }

    fn height(&self) -> usize {
        self.arena[0].len()
    }

    fn move_robot(&mut self, dir: Direction) {
        let (x, y) = self.robot_pos;
        let (to_x, to_y) = move_in_direction(dir, x, y);
        if self.arena[to_x][to_y].is_empty() {
            self.robot_pos = (to_x, to_y);
        } else if self.arena[to_x][to_y].is_box() && self.can_box_move(dir, to_x, to_y) {
            self.move_box(dir, to_x, to_y, Entity::Empty);
            self.robot_pos = (to_x, to_y);
        }
    }

    // We already check whether the move is possible in `move_robot`, so we don't need to do it again here
    fn move_box(&mut self, dir: Direction, x: usize, y: usize, replacement_entity: Entity) {
        let old_entity = self.arena[x][y];

        use Direction::*;
        use Entity::*;

        match (old_entity, dir) {
            (Box, _) | (BigBoxLeft | BigBoxRight, Left | Right) => {
                let (to_x, to_y) = move_in_direction(dir, x, y);
                self.move_box(dir, to_x, to_y, old_entity);
            }
            (BigBoxLeft, Up | Down) => self.move_big_box_up_or_down(dir, x, y),
            (BigBoxRight, Up | Down) => self.move_big_box_up_or_down(dir, x - 1, y),
            _ => (),
        }

        self.arena[x][y] = replacement_entity;
    }

    fn move_big_box_up_or_down(&mut self, dir: Direction, x: usize, y: usize) {
        let (_, to_y) = move_in_direction(dir, x, y);

        use Entity::*;

        for to_x in x - 1..x + 2 {
            if self.arena[to_x][to_y] == BigBoxLeft {
                self.move_big_box_up_or_down(dir, to_x, to_y);
            }
        }

        self.arena[x][to_y] = BigBoxLeft;
        self.arena[x + 1][to_y] = BigBoxRight;
        self.arena[x][y] = Empty;
        self.arena[x + 1][y] = Empty;
    }

    fn can_box_move(&self, dir: Direction, x: usize, y: usize) -> bool {
        let entity = self.arena[x][y];

        use Direction::*;
        use Entity::*;

        match (entity, dir) {
            (Box, _) | (BigBoxLeft | BigBoxRight, Left | Right) => {
                let (to_x, to_y) = move_in_direction(dir, x, y);
                self.can_box_move(dir, to_x, to_y)
            }
            (BigBoxLeft, Up | Down) => {
                let (to_x, to_y) = move_in_direction(dir, x, y);
                self.can_box_move(dir, to_x, to_y) && self.can_box_move(dir, to_x + 1, to_y)
            }
            (BigBoxRight, Up | Down) => {
                let (to_x, to_y) = move_in_direction(dir, x, y);
                self.can_box_move(dir, to_x, to_y) && self.can_box_move(dir, to_x - 1, to_y)
            }
            (Empty, _) => true,
            (Obstacle, _) => false,
        }
    }

    pub fn apply_movements(&mut self) {
        for dir in self.movements.clone() {
            self.move_robot(dir);
        }
    }

    pub fn sum_box_coordinates(&self) -> usize {
        self.arena
            .iter()
            .enumerate()
            .map(|(x, xs)| {
                xs.iter()
                    .enumerate()
                    .filter(|(_, entity)| **entity == Entity::Box || **entity == Entity::BigBoxLeft)
                    .map(move |(y, _)| y * 100 + x)
            })
            .flatten()
            .sum()
    }

    pub fn embiggen(&mut self) {
        use Entity::*;

        let mut bigger_arena = Vec::new();

        self.arena
            .iter()
            .map(|col| {
                col.iter()
                    .map(|entity| match entity {
                        Obstacle => (Obstacle, Obstacle),
                        Box => (BigBoxLeft, BigBoxRight),
                        _ => (Empty, Empty),
                    })
                    .unzip()
            })
            .for_each(|(left, right)| {
                bigger_arena.push(left);
                bigger_arena.push(right);
            });

        self.arena = bigger_arena;
        self.robot_pos.0 *= 2;
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if (x, y) == self.robot_pos {
                    buf.push('@');
                } else {
                    buf.push(self.arena[x][y].into());
                }
            }
            buf.push('\n');
        }
        writeln!(f, "{buf}")
    }
}

fn split_string<'a>(string: &'a str, split_at: &str) -> (&'a str, &'a str) {
    let divider = string.find(split_at).unwrap();

    (&string[0..divider], &string[divider..string.len()])
}
