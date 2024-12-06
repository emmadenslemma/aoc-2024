mod direction;
mod entity;
mod map;

use direction::Direction;
use entity::Entity;
use map::*;

use rayon::prelude::*;

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

    (0..map.width())
        .into_par_iter()
        .map(|x| {
            (0..map.height())
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
